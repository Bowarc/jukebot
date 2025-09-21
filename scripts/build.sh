#!/bin/bash

if [ "$1" = release ] || [ "$1" = r ]
then
  echo Compiling using release mode
  cargo build --release 
else
  echo Compiling using debug mode
  cargo build
fi

echo Done
