# Overview
Ping Spoofer is a simple CLI tool that artificially increases your ping systemwide.

## Installation
```bash
git clone https://github.com/Siriusmart/ping-spoofer && cd ping-spoofer && sh ./install.sh
```

## Commands:
```bash
ping-spoofer on [ms] [device]
ping-spoofer off [device]
ping-spoofer uninstall
ping-spoofer --help
ping-spoofer --version
```

Reference:
 * [ms] is the amount of milliseconds to increase your ping by.
 * [device] is the device to increase your ping on, can be found by running 'tc qdisc ls', it looks something like this: `eth0`/`lan0`/`wlan0`/...";