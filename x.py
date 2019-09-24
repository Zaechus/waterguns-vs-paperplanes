#!/usr/bin/env python3

from shutil import which
from subprocess import call
import os

if which("cargo") is None:
    print("Install Cargo before continuing")
    quit()

if which("npm") is None:
    print("Install npm before continuing")
    quit()

if which("tsc") is None:
    print("Install TypeScript before continuing: `npm install -g typescript`")
    quit()

if which("wasm-pack") is None:
    call(["cargo", "install", "wasm-pack"])

if os.path.exists("dist"):
    os.chdir("dist")
    for filename in os.listdir("."):
        if filename.endswith(".wasm"):
            os.remove(filename)
    os.chdir("..")

os.chdir("crate")
call(["wasm-pack", "build"])

os.chdir("..")
call(["npm", "install"])

os.chdir("src/ts")
for filename in os.listdir("."):
    if filename.endswith(".ts"):
        call(["tsc", filename, "--outDir", ".."])

os.chdir("../..")
call(["npx", "webpack"])
call(["cargo", "run"])
