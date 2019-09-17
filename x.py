#!/usr/bin/env python3

from subprocess import call
from shutil import rmtree
import os

if os.path.exists("dist"):
    rmtree("dist")

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
