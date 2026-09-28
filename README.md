hare-pico
=========

## Building

```bash
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo install flip-link --locked
cargo build
```
