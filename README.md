# rust-video
Realtime video rendering in Rust. Inspired by https://nullprogram.com/blog/2017/11/03/

Experiment about writing video data into stdout and playing it in a video player (mpv)
Was wondering if this would be the - in a sense - the simplest way to put pixels on screen
Later this idea evolved into ![imagesink](https://github.com/kirinokirino/imagesink)

## Running
rust-video outputs to stdout, so you can pipe into a video player like mpv:
```bash
cargo run --release | mpv --untimed --no-input-default-bindings --osc=no -
cargo run --release | mpv  --no-input-default-bindings --osc=no --fps=10 --no-correct-pts --no-cache -
```
