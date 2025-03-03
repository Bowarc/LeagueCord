#!/bin/bash

if [ "$1" = release ] || [ "$1" = r ]
then
  echo Compiling back using release mode
  cargo build --release 
else
  echo Compiling back using debug mode
  cargo build
fi

echo Done
