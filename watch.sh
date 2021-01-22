#!/bin/sh

set -x

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
brunch watch &

cargo build

if [ $? -ne 0 ]; then
  echo "Fix your broken build, man."
  return -1
fi

export RUST_LOG=debug
cargo run -- $@
