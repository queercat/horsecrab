# 🐴🦀
This is a WIP forum that doesn't use any JavaScript! And it shouldn't suck either. It uses SSR templates powered by rshtml.

# Goals
- Should be lightweight and fast.
- Privacy first.
- It should look good.
- It should be accessible and modern.
- Shouldn't be a nightmare to host.

# Progress
Current work is tracked in [PROJECT.md](/PROJECT.md)

# Requirements
- Rust :D

# Quickstart
```sh
cp .env.dev .env
touch database.db # ignore if you're using postgres or mysql
cargo run -p migration fresh
cargo run
```
