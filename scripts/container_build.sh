#!/bin/bash

set -e

echo Building image
podman build -t jukebot:latest .
