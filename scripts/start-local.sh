#!/bin/bash

NUM_INSTANCES=10
CMDS=()

for i in $(seq 1 $NUM_INSTANCES); do
    PORT=$((8998 + i))
    CMD="PORT_HTTP=$PORT cargo run"
    CMDS+=("\"$CMD\"")
done

JOINED=$(
    IFS=" "
    echo "${CMDS[*]}"
)
eval nodemon -e rs -x "concurrently $JOINED"
