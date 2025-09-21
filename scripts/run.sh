#!/bin/bash

set -e

echo Running Jukebot

if [ "$1" = release ] || [ "$1" = r ]
then
  echo Running using release mode
  cargo run --release
else
  echo Running using debug mode
  cargo run
fi
