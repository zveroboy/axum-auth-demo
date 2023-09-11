```
cargo watch -c -q -w src/ -x run
```

```
cargo watch -c -q -w tests/ -x "test --tests -q -- --nocapture router_hello"
```

```
cargo test --tests -q -- --nocapture router_hello
```
