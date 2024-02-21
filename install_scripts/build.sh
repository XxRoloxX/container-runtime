echo "Add static build target"
rustup target add x86_64-unknown-linux-musl
echo "Building from source"
cargo build --target=x86_64-unknown-linux-musl
