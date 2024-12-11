#!/usr/bin/env python3

import docker
from dotenv import load_dotenv
import main
import os
import subprocess
from pprint import pprint

CWD = os.getcwd()

def get_arch():
        arch =  os.uname().machine
        if arch == "armv7l":
            return "32"
        elif arch == "arm64" or arch == "x86_64":
            return "64"

def setup():
    main.Main().main()
    arch = get_arch()
    
    if arch == "32":
        subprocess.run([
            "docker",
            "run",
            "-d",
            "-l mtvtserver:latest",
            "-v",
            "/usr/share/MTV/thumbnails:/usr/share/nginx/html:ro",
            "-p",
            "9999:80",
            "arm32v7/nginx:bookworm"
        ])
    elif arch == "64":
        subprocess.run([
            "docker",
            "run",
            "-d",
            "-l mtvtserver:latest",
            "-v",
            "/usr/share/MTV/thumbnails:/usr/share/nginx/html:ro",
            "-p",
            "9999:80",
            "nginx:bookworm"
        ])

if __name__ == "__main__":
    load_dotenv()
    setup()