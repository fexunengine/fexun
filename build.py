#!/usr/bin/env python3
import subprocess
import sys
import platform
import shutil
from pathlib import Path

WORKSPACE_ROOT = Path(__file__).parent
RUNTIME_DIR = WORKSPACE_ROOT / "fexun-runtime"
OUTPUT_DIR = WORKSPACE_ROOT / "target"

SYSTEM = platform.system()

PLATFORMS = {
    "Linux": {
        "target": "x86_64-unknown-linux-gnu",
        "library_name": "libfexun_runtime.so",
        "executable_name": "fexun-game",
        "compiler": "gcc",
        "launcher_flags": ["-Wl,-rpath,$ORIGIN"],
        "package_name": "linux",
    },
    "Windows": {
        "target": "x86_64-pc-windows-msvc",
        "library_name": "fexun_runtime.dll",
        "executable_name": "fexun-game.exe",
        "compiler": "cl.exe",
        "launcher_flags": [],
        "package_name": "windows",
    },
}

def get_platform_config():
    if SYSTEM not in PLATFORMS:
        print(f"Error: Unsupported platform: {SYSTEM}")
        sys.exit(1)
    return PLATFORMS[SYSTEM]

def build_runtime(config, release=False):
    cmd = ["cargo", "build", "--manifest-path", str(RUNTIME_DIR / "Cargo.toml"),
           "--target", config["target"]]
    if release:
        cmd.append("--release")
    
    profile = "release" if release else "debug"
    print(f"Building fexun-runtime ({profile}) for {config['target']}...")
    result = subprocess.run(cmd, cwd=WORKSPACE_ROOT)
    if result.returncode != 0:
        print("Build failed")
        sys.exit(1)

def generate_launcher_c(output_path):
    launcher_c = """
#include <stdio.h>

extern void fexun_run(void);

int main(int argc, char* argv[]) {
    printf("Starting Fexun Engine...\\n");
    fexun_run();
    return 0;
}
"""
    output_path.write_text(launcher_c)

def create_executable_linux(config, release=False):
    profile = "release" if release else "debug"
    build_dir = OUTPUT_DIR / config["target"] / profile
    runtime_lib = build_dir / config["library_name"]
    output_exe = build_dir / config["executable_name"]
    launcher_c = build_dir / "launcher.c"

    if not runtime_lib.exists():
        print(f"Error: Runtime library not found: {runtime_lib}")
        sys.exit(1)

    generate_launcher_c(launcher_c)

    compile_cmd = [
        config["compiler"],
        str(launcher_c),
        "-o", str(output_exe),
        f"-L{build_dir}",
        "-lfexun_runtime",
    ] + config["launcher_flags"]
    
    if release:
        compile_cmd.insert(1, "-O3")

    print("Linking launcher...")
    result = subprocess.run(compile_cmd)
    if result.returncode != 0:
        print("Linking failed")
        sys.exit(1)

    return output_exe

def create_executable_windows(config, release=False):
    profile = "release" if release else "debug"
    build_dir = OUTPUT_DIR / config["target"] / profile
    runtime_lib = build_dir / config["library_name"].replace(".dll", ".lib")
    output_exe = build_dir / config["executable_name"]
    launcher_c = build_dir / "launcher.c"

    if not runtime_lib.exists():
        print(f"Error: Import library not found: {runtime_lib}")
        sys.exit(1)

    generate_launcher_c(launcher_c)

    compile_cmd = [
        config["compiler"],
        str(launcher_c),
    ]
    
    if release:
        compile_cmd.append("/O2")
    
    compile_cmd.extend([
        f"/link",
        f"/OUT:{output_exe}",
        str(runtime_lib),
    ])

    print("Linking launcher...")
    result = subprocess.run(compile_cmd, cwd=build_dir)
    if result.returncode != 0:
        print("Linking failed")
        sys.exit(1)

    return output_exe

def main():
    release = "--release" in sys.argv
    run = "--run" in sys.argv
    config = get_platform_config()
    
    build_runtime(config, release=release)

    if SYSTEM == "Linux":
        exe_path = create_executable_linux(config, release=release)
    elif SYSTEM == "Windows":
        exe_path = create_executable_windows(config, release=release)
    else:
        print(f"Error: Executable creation not implemented for {SYSTEM}")
        sys.exit(1)

    print(f"Build complete: {exe_path}")

    if run:
        print(f"Running: {exe_path}\n")
        result = subprocess.run([str(exe_path)])
        if result.returncode != 0:
            print(f"Execution failed with exit code {result.returncode}")
            sys.exit(result.returncode)

if __name__ == "__main__":
    main()
