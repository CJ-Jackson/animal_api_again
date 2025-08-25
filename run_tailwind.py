#!/usr/bin/env python3
import os
import subprocess
from pathlib import Path

os.chdir(os.path.dirname(os.path.abspath(__file__)))

map_css: dict = {
    "ui/assets/tailwind/main.css": "ui/assets/styling/main.css"
}

for key, value in map_css.items():
    subprocess.run(['npx', '@tailwindcss/cli', '-i', key, '-o', value])
    print(f"{key} -> {value}")
