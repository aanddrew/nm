#!/bin/bash

cd libnm
cargo build
cd ..
cd nm_test
if [ "$1" ];
then
    cargo run "$1"
else
    cargo run
fi
cd ..
