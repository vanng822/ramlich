# ramlich
Rust version of https://github.com/vanng822/vncalendar

# cross compiling

https://rust-lang.github.io/rustup/cross-compilation.html

## install

```bash
> rustup target add x86_64-unknown-linux-gnu
> brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
```
## Cargo.toml config

```
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```

## build
```bash
> TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu
```