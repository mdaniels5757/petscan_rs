#!/bin/bash
# Get latest code
git pull

# Update
cargo update

# Build new server binary
rm ../cargo.out ../cargo.err

jsub -once -sync -cwd -mem 2048m cargo build --release

sleep 5

tail -f ../cargo.out ../cargo.err

# Get restart code from config file
code=`jq -r '.["restart-code"]' config.json`

# Build restart URL
url="http://petscan-md.toolforge.org/?restart=$code"

# Restart server
curl -s -o /dev/null $url
#sleep 1
#screen -r pts-0.petscan4
#/usr/sbin/service ./target/release/petscan_rs restart
#webservice restart