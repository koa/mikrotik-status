#!/bin/sh

cd frontend &&\
trunk build --release &&\
cd .. &&\
cargo build -p mikrotik-status --release