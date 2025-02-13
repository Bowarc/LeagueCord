#!/bin/bash

mode=debug # debug, release

echo Compiling back
if [ "$mode" = release ]
then
  cargo build --release 
else
  cargo build
fi

echo Done
