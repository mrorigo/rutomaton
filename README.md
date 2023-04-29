
# Cellular Automaton in Rust

Inspired by a [Tsoding session](https://www.youtube.com/watch?v=Hz_13P7lRoA)

## Dependencies

You need SDL2, see [instructions here](https://blog.logrocket.com/using-sdl2-bindings-rust/)

On MacOS, ensure you have `LIBRARY_PATH` environment variable set to locate your `libSDL2.a`, or linking will fail.


## Running the examples

```bash
    cargo run --release --example game_of_life
    cargo run --release --example wireworld
    cargo run --release --example seeds
    cargo run --release --example brians_brain
```
