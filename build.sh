#!/bin/bash
cargo build --release
cp target/release/dessin-scale ./
gpg --detach-sign -a dessin-scale