cargo watch -s "clear && sh ./scripts/clean_back.sh & sh ./scripts/build_back.sh && cargo r" -w ./src -w ./Rocket.toml --why
