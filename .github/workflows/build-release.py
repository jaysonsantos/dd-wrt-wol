#!/usr/bin/env python3
import pathlib
import subprocess
import shlex
import shutil

PLATFORMS="x86_64-unknown-linux-gnu armv7-unknown-linux-gnueabihf aarch64-unknown-linux-gnu"
DOCKER_PLATFORM= {"x86_64": "linux/amd64", "armv7": "linux/arm/v7", "aarch64": "linux/arm64"}
TARGET_FOLDER = pathlib.Path("target")
CACHE_FOLDER = ".cache"
for platform in PLATFORMS.split():
    docker_platform=DOCKER_PLATFORM[platform.split('-')[0]]
    cargo = 'cargo' if platform.startswith('x86_64') else 'cross'
    subprocess.check_call(shlex.split(f'rustup target add {platform}'))
    subprocess.check_call(shlex.split(f'{cargo} build --release --target="{platform}"'))
    docker_target = CACHE_FOLDER / pathlib.Path(docker_platform)
    docker_target.mkdir(parents=True, exist_ok=True)
    print((TARGET_FOLDER / platform / "release"))
    for binary in (TARGET_FOLDER / platform / "release").glob("dd-wrt-*"):
        destination = docker_target / binary.name
        print(f'Copying {binary} to {destination}')
        shutil.copy2(binary, destination)
