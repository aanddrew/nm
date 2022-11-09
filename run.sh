#!/bin/bash

cd libnm
cargo build
cd ..
cd nm_test
cargo run
cd ..
