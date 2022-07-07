#!/usr/bin/python3
"""Check LF line ending and empty new line in the end"""

from os import listdir
from os.path import isfile, isdir, join
import sys

ignore = [
    "build",
    "release",
    "node_modules",
    ".git",
    "__pycache__",
    ".vscode",
    "target",
    "binaryen-version_105",
    "pkg",
    "wasmlib",
    "docs"
]

ignore_exts = [
    "CNAME",
    ".png",
    ".gif",
    ".vsix",
    ".celer",
    ".ico",
    ".ps1",
    ".wasm",
    ".tar.gz",
    ".otf",
    ".svg"
]

ROOT = "."

if len(sys.argv) > 1:
    VERBOSE = sys.argv[1] == "--verbose"
else:
    VERBOSE = False


def lint_path(rel_path):
    """Lint path (file or directory)"""
    if isfile(rel_path):
        lint_file(rel_path)
    elif isdir(rel_path):
        for subpath in listdir(rel_path):
            if should_lint_subpath(subpath):
                lint_path(join(rel_path, subpath))

def should_lint_subpath(subpath):
    """Check if a subpath should be processed or ignored"""
    if subpath in ignore:
        return False
    for ignore_ext in ignore_exts:
        if subpath.endswith(ignore_ext):
            return False

    return True

bad_files = []

def lint_file(file_path):
    """Lint file"""
    if VERBOSE:
        print(f"Checking {file_path}")
    with open(file_path, 'r', encoding="utf-8", newline="") as file:
        last_line = None
        while True:
            line = file.readline()
            if not line:
                break
            if line.find("\r") != -1:
                bad_files.append(("Line Ending", file_path))
                return
            last_line = line
        if last_line is not None:
            if last_line[-1] != "\n":
                if VERBOSE:
                    print(f"lastline={last_line} length={len(last_line)}")
                bad_files.append(("Needs trailing new line", file_path))
            elif last_line[0] == "\n":
                bad_files.append(("Too many trailing new lines", file_path))

lint_path(ROOT)

if len(bad_files) > 0:
    for reason, path in bad_files:
        print(f"{path}: {reason}")
    sys.exit(1)
