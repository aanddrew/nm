#!/bin/bash

cd libnm
cargo build
cd ..
cd nm_ui
trunk serve
cd ..
