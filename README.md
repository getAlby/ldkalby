# Setup

```sh
cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.2.1+v0.25.0
```

## Run examples

```sh
cd examples
```

### GetInfo

```sh
MNEMONIC="YOUR TWELVE WORD MNEMONIC HERE" cargo run --bin get-info
```

## Generate bindings

```sh
cargo build --release
uniffi-bindgen-go src/ldkalby.udl -o ffi/golang -c ./uniffi.toml
cp target/release/libldkalby_bindings.so ffi/golang/ldkalby
```

## Run tests

```sh
cp -r ffi/golang/ldkalby tests/bindings/golang/
cargo test -- --nocapture
```

## Build and copy to NWC

Make sure to set `YOUR_NWC_NEXT_DIR`

```sh
cargo build --release && uniffi-bindgen-go src/ldkalby.udl -o ffi/golang -c ./uniffi.toml && cp target/release/libldkalby_bindings.so ffi/golang/ldkalby && cp ffi/golang/ldkalby YOUR_NWC_NEXT_DIR -r
```

## Consume from go app

Copy `libldkalby_bindings.so` into `ldkalby` folder and then copy `ldkalby` folder into NWC app.

Import with `import ("github.com/getAlby/nostr-wallet-connect/ldkalby")`

And then you can call functions e.g. `ldkalby.GetInfo()`

```sh
CGO_LDFLAGS="-lldkalby_bindings -L./ldkalby -Wl,-rpath,./ldkalby" go run .
```
