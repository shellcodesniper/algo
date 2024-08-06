#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <project_name>"
    exit 1
fi

cp -rf template "$1"

# rename cargo.toml name
sed -i '' "s/template/algo_$1/g" "$1/Cargo.toml"

