import subprocess
import time
from pathlib import Path
import json
from pprint import pprint
import statistics

def flash_board(target, profile):
    subprocess.run(
        [
            "cargo", "flash", 
            "--chip", "STM32F767ZITx",
           "--", 
            "--profile", profile,
            "--package", f"bench-{target}",
            "-Zjson-target-spec"
        ],
        # stdout=subprocess.DEVNULL, 
        # stderr=subprocess.DEVNULL
    )

def flash_board_miosix(target):
    subprocess.run(
        [
            "cmake", "-S.", "-Bbuild", "-GNinja"
        ],
        cwd=Path("./miosix/benches") / target
    )
    
    subprocess.run(
        [
            "ninja", "program"
        ],
        cwd=Path("./miosix/benches") / target / "build"
    )

def reboot_board():
    subprocess.run(
        ["st-flash", "reset"],
        stdout=subprocess.DEVNULL, 
        stderr=subprocess.DEVNULL
    )

def parse_output(output):
    lines = output.splitlines()
    lines = lines[2:]
    captures = {}
    for line in lines:
        ch = line[:2]
        data = line[3:].replace(" ", "")
        data = map(lambda char: char != '0', data)
        captures.setdefault(ch, []).extend(data)
    
    return captures

def start_capture(chs, time):
    proc = subprocess.Popen(
        [
            "sigrok-cli", 
            "--driver", "fx2lafw", 
            "-C", ",".join(chs), 
            "--time", str(time), 
            "-O", "bits",
            "--config", "samplerate=24m",
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True
    )
    
    return lambda: parse_output(proc.communicate()[0])

def detect_pulses(signals):
    pulses = []
    count = 0
    for point in signals:
        if count > 0:
            if point:
                count += 1
            else:
                pulses.append(count)
                count = 0
        else:
            if point:
                count += 1

    return pulses

def detect_inversions(signals):
    inv = []
    last = signals[0]
    for i, point in enumerate(signals):
        if point != last:
            inv.append(i)
            last = point
            
    return inv

def detect_pulse_delay(base, delayed):
    inv_base = detect_inversions(base)
    inv_delayed = detect_inversions(delayed)
    
    # We got an extra pulse in delayed
    if inv_base[0] > inv_delayed[0]:
        inv_delayed.pop(0)

    # We missed the last delayed
    if inv_base[-1] > inv_delayed[-1]:
        inv_base.pop(-1)

    assert len(inv_base) == len(inv_delayed)
    return list(map(lambda a: a[1] - a[0], zip(inv_base, inv_delayed)))

def capture_pulse():
    # Prepare capture
    out = start_capture(["D0"], 2000)
    
    time.sleep(0.1)
    reboot_board()
    
    out = out()
    pulses = detect_pulses(out["D0"])
    pulses = list(map(lambda pulse: pulse / 24000000, pulses))
    print(pulses)
    return pulses[-1]

def capture_average():
    pulse = []
    for _ in range(0, 10):
        pulse.append(capture_pulse())
    return pulse

def capture_pulse_delay():
    out = start_capture(["D0", "D1"], 2000)
    
    time.sleep(0.1)
    reboot_board()
    
    out = out()
    pulses = detect_pulse_delay(out["D0"][12000000:], out["D1"][12000000:])
    pulses = list(map(lambda pulse: pulse / 24000000, pulses))
    return pulses

TARGETS = [
    "yield",
    "condvar",
    "mutex-lock",
    "mutex-unlock",
    "mutex-uncont",
    "wake-from-irq"
]

MIOSIX_TARGETS = [
    "yield",
    "condvar",
    "mutex-lock",
    "mutex-unlock",
    "mutex-uncont",
    "wake-from-irq"
]

PROFILE = [
    "release",
    "releaseplus"
]

def run():
    results = {}

    results["miosix"] = {}
    for profile in PROFILE:
        results[f"eva/{profile}"] = {}

    # for profile in PROFILE:
    #     for target in TARGETS:
    #         flash_board(target, profile)
    #         time.sleep(3)
    #         results[f"eva/{profile}"][target] = capture_average()
    
    # for target in MIOSIX_TARGETS:
    #     flash_board_miosix(target)
    #     time.sleep(3)
    #     results["miosix"][target] = capture_average()

    flash_board("irq-latency", "releaseplus")
    time.sleep(3)
    delays = capture_pulse_delay()
    results["eva/releaseplus"]["irq-latency"] = {
        "mean": statistics.mean(delays) * 1000000000,
        "stdev": statistics.stdev(delays) * 1000000000
    }
    
    flash_board_miosix("irq-latency")
    time.sleep(3)
    delays = capture_pulse_delay()
    results["miosix"]["irq-latency"] = {
        "mean": statistics.mean(delays) * 1000000000,
        "stdev": statistics.stdev(delays) * 1000000000
    }

    print(json.dumps(results))
    
run()