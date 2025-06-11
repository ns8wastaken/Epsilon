#!/usr/bin/bash

# Set engine paths
engine1="./target/release/Epsilon"
engine2="./Epsilon_v2_alphabeta"

# Set log file name
logfile="debug_log.txt"

# Run cutechess-cli with debug logging
~/Downloads/cutechess-cli \
    -engine cmd=$engine1 name=new \
    -engine cmd=$engine2 name=old \
    -each proto=uci tc=inf debug=on \
    -games 200 -concurrency 16 \
    -sprt elo0=0 elo1=5 alpha=0.05 beta=0.1
