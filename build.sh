#!/bin/sh

cargo build -p implementation --release &&\
cd frontend &&\
trunk clean &&\
trunk build --release &&\
cd .. &&\
cargo build -p backend --release &&
docker build . -t docker.berg-turbenthal.ch/mikrotik-status:latest