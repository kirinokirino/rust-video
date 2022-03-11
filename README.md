# rust-video
Realtime video rendering in Rust. Inspired by https://nullprogram.com/blog/2017/11/03/

## Running
rust-video outputs to stdout, so you can pipe into a video player like mpv:
```bash
cargo run --release | mpv --untimed --no-input-default-bindings --osc=no -
cargo run --release | mpv  --no-input-default-bindings --osc=no --fps=10 --no-correct-pts --no-cache -
```
