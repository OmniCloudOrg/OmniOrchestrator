#!/bin/bash

echo "Starting cluster setup..."

# Function to handle cleanup on script exit
cleanup() {
    echo "Cleaning up..."
    jobs -p | xargs -r kill
    exit
}

# Set up trap for cleanup
trap cleanup EXIT INT TERM

# Read ports from config.json using jq
# Make sure jq is installed: sudo apt-get install jq
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install it first."
    exit 1
fi

# Get array of ports from instances
ports=$(jq -r '.instances[].port' config.json)

# Process each port
for port in $ports; do
    echo "Processing port: $port"
    
    # Update the top-level port in config.json
    jq --arg port "$port" '.port = ($port | tonumber)' config.json > config.json.tmp && mv config.json.tmp config.json
    
    # Start the instance
    echo "Starting cargo for port $port"
    cargo run &
    
    # Wait before starting next instance
    sleep 5
done

echo
echo "Cluster setup complete. Instances are running in background."
echo "Use 'ps aux | grep cargo' to view running processes."
echo

# Wait for all background processes
wait