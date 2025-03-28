#!/bin/bash

set -e

echo Running server

if [ "$1" = release ] || [ "$1" = r ]
then
  echo Running server using release mode
  cargo run --release
else
  echo Running server using debug mode
  cargo run
fi
