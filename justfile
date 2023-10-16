run-main: 
  cargo run -q --bin main --config config.docker.toml
watch-main: 
  cargo watch -c -q -w src/ -w .cargo/ -x "run --bin main --config config.docker.toml"

run-dev-db: 
  cargo run -q --bin dev_db --config config.docker.toml

test:
  cargo watch -c -q -w src/ -w tests/ -x "test --test hello -q -- --nocapture"
  
test-all:
  cargo watch -c -q -w src/ -w tests/ -x "test --tests -q -- --nocapture router_hello"