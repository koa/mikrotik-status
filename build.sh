#!/bin/sh

cd implementation && cargo build --release &&\
cd ../frontend &&\
trunk build --release &&\
cd ../backend &&\
cargo build --release