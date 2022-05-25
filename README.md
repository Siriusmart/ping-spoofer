# Overview
A simple CLI tool that artificially increases your ping systemwide.

## Installation
```bash
git clone https://github.com/Siriusmart/ping-spoofer && cd ping-spoofer && sh ./install.sh
```

## Commands:
```bash
ping-spoofer on [ms] [device]
ping-spoofer off [device]
ping-spoofer uninstall
```

for more information, run:
```bash
ping-spoofer help
```

## Reference:
 * [ms] is the amount of milliseconds to increase your ping by.
 * [device] is the device to increase your ping on, can be found by running 'tc qdisc ls', it looks something like this: `eth0`/`lan0`/`wlan0`/...";

## Bypass Root
When installing ping-spoofer, you can set the program to be allowed to run as root. (No need to re-enter your sudo password every time)

## License
Ping Spoofer is licensed under the almight GPLv3 license, you are free to use, study, modify, and redistribute this software under the same license as the original work.