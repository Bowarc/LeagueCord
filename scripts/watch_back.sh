cargo watch -s "clear && sh ./scripts/clean.sh & sh ./scripts/build_back.sh && cargo r -p back" -w ./back/src -w ./Rocket.toml --why
