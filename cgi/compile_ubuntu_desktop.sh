rm -rf ./target
cargo build --release
sudo mv ./target/release/numerologie_core /usr/lib/cgi-bin/numerologie_core.cgi
