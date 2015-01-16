# cjdrs – cjdns implementation in Rust

[![Build Status](https://travis-ci.org/Randati/cjdrs.svg)](https://travis-ci.org/Randati/cjdrs)

cjdrs will be an implementation of the [cjdns](https://github.com/cjdelisle/cjdns/) protocol. Currently very much incomplete and not usable in any way.

* [cjdns on GitHub](https://github.com/cjdelisle/cjdns/)
* [cjdns on Wikipedia](https://en.wikipedia.org/wiki/Cjdns)

## Getting it up and running
```shell
# Install Rust and Cargo
curl -s https://static.rust-lang.org/rustup.sh | sudo sh

# Install libsodium
wget https://download.libsodium.org/libsodium/releases/libsodium-1.0.2.tar.gz
tar -zxvf libsodium-1.0.2.tar.gz
cd libsodium-1.0.2
./configure --prefix=/usr
make && make check
sudo make install
cd ..

# Clone repository
git clone https://github.com/Randati/cjdrs.git
cd cjdrs

cargo build               # Build
cargo test                # Run tests
./target/cjdrs init       # Generate configuration file
editor cjdrs.conf         # Edit configuration
sudo ./target/cjdrs run   # Run
```
