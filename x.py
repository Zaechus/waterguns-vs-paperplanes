#!/usr/bin/env python3

from subprocess import call
import os

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
