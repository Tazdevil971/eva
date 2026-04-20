import subprocess
import time
import pprint
from pathlib import Path

def flash_board(target, profile):
    subprocess.run(
        [
            "cargo", "flash", 
            "--chip", "STM32F767ZITx",
           "--", 
            "--profile", profile,
            "--package", target,
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
    pulse = 0
    for _ in range(0, 3):
        pulse += capture_pulse()
    return pulse / 3

TARGETS = [
    "bench-yield",
    "bench-condvar",
    "bench-mutex-lock",
    "bench-mutex-unlock",
    "bench-mutex-uncont",
    "bench-wake-from-irq"
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
    
    for target in TARGETS:
        for profile in PROFILE:
            flash_board(target, profile)
            time.sleep(1)
            raw = capture_pulse()
            us = raw * 1000000
            cycles = int(raw * 216000000)
            
            results[(target, profile)] = f"{us}us / {cycles}cycles"
            pprint.pprint(results)
    
    for target in MIOSIX_TARGETS:
        flash_board_miosix(target)
        time.sleep(1)
        raw = capture_pulse()
        us = raw * 1000000
        cycles = int(raw * 216000000)
        
        results[(target, "miosix")] = f"{us}us / {cycles}cycles"
        pprint.pprint(results)
    
    pprint.pprint(results)
    
run()