#!/bin/bash

FILE="./target/release/ping-spoofer"

echo "Building..."

cargo build --release

echo "Would you like to allow ping spoofer to run as root without password? (Y/n)"
read answer

case $answer in
    y) sudo chown root:root "$FILE"
    sudo chmod 4755 "$FILE";;
    n) echo "OK, not running as root
    ";;
    *) echo "Defaulting to running as root"
        sudo chown root:root "$FILE"
        sudo chmod 4755 "$FILE";;
esac

echo "Would you like to install the binary to on of the following locations?
    1. /bin (default)
    2. ~/.cargo/bin"
read answer

case $answer in
    1) sudo mv "$FILE" /bin/ping-spoofer -f;;
    2) mv "$FILE" ~/.cargo/bin/ping-spoofer -f;;
    *) sudo mv "$FILE" /bin/ping-spoofer -f;;
esac

cd .. && rm -rf ./ping-spoofer