#!/bin/bash

export PATH="/home/ma1y0/.cargo/bin:$PATH"

if [ -x "$(command -v cargo)" ]; then
    echo "true"
else 
    echo "false"
fi

echo "$(command -v curl)"
echo "$PATH"

echo "$(command ls -R)"
