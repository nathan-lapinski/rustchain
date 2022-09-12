## Rustchain
This is an implementation of a small, Bitcoin-like blockchain in Rust. In the future, it may use WASM to enable web compatibility.

Currently, it implements a proof-of-work consensus mechanism similar to what Bitcoin does. Next steps will be:
- Add a small test network using sockets
- Update nodes to take the longest verifiable chain

After that, this will be updated to support a proof-of-stake consensus mechanism.

## Run
```
cargo build && cargo run
```
