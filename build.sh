#!/bin/sh

cd backend && cargo build --release &&\
cd ../frontend &&\
trunk build --release &&\
cd ../binary &&\
cargo build --release