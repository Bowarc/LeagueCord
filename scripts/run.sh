#!/bin/bash

set -e

mode=debug # debug, release

echo Running server
if [ "$mode" = release ]
then
  cargo run --release
else
  cargo run
fi
