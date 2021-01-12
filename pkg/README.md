# bevy_egui_web_showcase

An example project running on [bevy_egui](https://github.com/mvlabat/bevy_egui).

Live at: https://mvlabat.github.io/bevy_egui_web_showcase/index.html

## Running

Prerequisites:
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [basic-http-server](https://github.com/brson/basic-http-server) (or any other http server to serve index.html)

```sh
wasm-pack build --target web --release
basic-http-server
```
