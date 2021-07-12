#!/bin/bash

docker build -f ./build/Dockerfile . -t colinjfw/snake:latest

id=$(docker create colinjfw/snake:latest)
docker cp $id:/tmp/snake/target/x86_64-unknown-linux-gnu/release/snake ./build/snake
docker rm -v $id
