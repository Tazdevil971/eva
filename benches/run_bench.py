import subprocess
import time
from pathlib import Path
import json

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
    lines = map(lambda line: line[3:], lines)
    lines = map(lambda line: line.replace(" ", ""), lines)
    lines = "".join(lines)
    return list(map(lambda char: char != '0', lines))

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

def capture_pulse():
    # Prepare capture
    proc = subprocess.Popen(
        [
            "sigrok-cli", 
            "--driver", "fx2lafw", 
            "-C", "D0", 
            "--time", "2000", 
            "-O", "bits",
            "--config", "samplerate=24m",
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True
    )
    
    time.sleep(0.1)
    reboot_board()
    
    outs, _ = proc.communicate()
    pulses = detect_pulses(parse_output(outs))
    pulses = list(map(lambda pulse: pulse / 24000000, pulses))
    print(pulses)
    return pulses[-1]

def capture_average():
    pulse = []
    for _ in range(0, 10):
        pulse.append(capture_pulse())
    return pulse

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

    for profile in PROFILE:
        results[f"eva/{profile}"] = {}
        for target in TARGETS:
            flash_board(target, profile)
            time.sleep(3)
            results[f"eva/{profile}"][target] = capture_average()
    
    for target in MIOSIX_TARGETS:
        flash_board_miosix(target)
        results["miosix"][target] = capture_average()

    print(json.dumps(results))
    
run()