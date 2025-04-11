#/bin/bash


set -e

if [ "$1" = test ] 
then
  echo Building image for test environement
  docker build --build-arg BOT_ENV=./test.env --build-arg ROCKET_CFG=./test.Rocket.toml -t leaguecord:test .
else
  echo Building image for production environement
  docker build --build-arg BOT_ENV=./.env --build-arg ROCKET_CFG=./Rocket.toml -t leaguecord:prod .
fi
