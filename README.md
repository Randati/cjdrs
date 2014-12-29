# cjdrs – cjdns implementation in Rust

[![Build Status](https://travis-ci.org/Randati/cjdrs.svg)](https://travis-ci.org/Randati/cjdrs)

cjdrs will be an implementation of the [cjdns](https://github.com/cjdelisle/cjdns/) protocol. Currently very much incomplete and not usable in any way.

## Getting it up and running
```shell
# Install Rust and Cargo
curl -s https://static.rust-lang.org/rustup.sh | sudo sh

# Install libsodium
wget http://download.libsodium.org/libsodium/releases/libsodium-1.0.1.tar.gz
tar -zxvf libsodium-1.0.1.tar.gz
cd libsodium-1.0.1
./configure --prefix=/usr
make && make check
sudo make install
cd ..

# Clone repository
git clone https://github.com/Randati/cjdrs.git
cd cjdrs

cargo build           # Build
cargo test            # Run tests
sudo ./target/cjdrs   # Run
```
