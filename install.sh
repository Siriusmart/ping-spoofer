#!/bin/bash

FILE="./src/main.rs"
FILE_NO_EXT="${FILE%.*}"

rustc "$FILE" -o "$FILE_NO_EXT"
sudo chown root:root "$FILE_NO_EXT"
sudo chmod 4755 "$FILE_NO_EXT"

sudo mv ./src/main /bin/ping-spoofer

cd .. && rm -rf ./ping-spoofer