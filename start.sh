#!/bin/bash

# Function to update JSON configuration
update_config() {
    local port=$1
    
    # Use jq to update the configuration
    # 1. Update the top-level port
    # 2. Update each instance port sequentially starting from the given port
    cat > config.json << EOF
$(jq --argjson port "$port" '
    .port = $port |
    .instances |= map(
        .port = ($port + index)
    )
' config.json)
EOF
}

# Function to start a new instance
start_instance() {
    local port=$1
    echo "Starting instance on port $port"
    cargo run &
    sleep 2  # Give some time for the instance to start
}

# Main execution
echo "Setting up cluster..."

# Read the available ports from the instances array
ports=($(jq -r '.instances[].port' config.json | sort -n))

# Start instances for each port
for port in "${ports[@]}"; do
    echo "Configuring for port $port"
    update_config "$port"
    start_instance "$port"
done

echo "Cluster setup complete. All instances are running in the background."
echo "Use 'jobs' to see running processes"
echo "Use 'fg %N' to bring a specific process to foreground"