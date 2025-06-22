"""Runs uking-relocate to make the mini image based on test results"""
import subprocess

START = "0x2044400000"
SDK = "data/botw150/sdk.elf"
OUTPUT = "data/program-mini.bfi"

with open("trace.txt", "r", encoding="utf-8") as f:
    lines = " ".join(f.read().split("\n"))
args = [x.strip() for x in lines.split(" ") if x.strip()]
subprocess.check_call(["uking-relocate", SDK, "-s", START, "-o", OUTPUT]+args)
