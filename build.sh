#!/bin/bash

echo "----- building server -----"
cargo build --release
echo "----- done server -----"

echo "----- building website -----"
npm install
npm run build
echo "----- done website -----"