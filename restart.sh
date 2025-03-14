#!/bin/bash
# Kill any existing server process
pkill cybrdelic-portfolio || true
sleep 1
# Run the server
cargo run