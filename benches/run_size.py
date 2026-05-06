import subprocess
import pprint
from pathlib import Path

def compile_example(target, profile):
    subprocess.run(
        [
            "cargo", "build",
            "--profile", profile,
            "--package", target,
            "-Zjson-target-spec"
        ]
    )
    
def get_code_size(target, profile):
    path = Path("../target/thumbv7em-eva-eabihf") / profile / target
    
    proc = subprocess.Popen(
        [ "arm-none-eabi-size", "-A", path ], 
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True
    )
    
    outs, _ = proc.communicate()
    return str(outs)

TARGETS = [
    "eva-hello-world",
    "eva-hello-world-std",
    "eva-hello-world-c-libc",
    "eva-hello-world-cpp-libc",
]

PROFILE = [
    "release",
    "releaseplus"
]

def run():
    results = {}

    for target in TARGETS:
        for profile in PROFILE:
            compile_example(target, profile)
            out = get_code_size(target, profile)
            results[(target, profile)] = out
    
    
    
    for (target, profile), value in results.items():
        print(f"{target} / {profile}:")
        print(value)
    
run()