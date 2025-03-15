# Omni Orchestrator

A powerful, scalable application orchestration platform that serves as the Raft consensus cluster API for OmniCloud internal services.

## Overview

Omni Orchestrator is a core component of the OmniCloud ecosystem designed to sit in front of internal services and manage them effectively. Using a Raft consensus-based cluster approach, it maintains platform health and resilience while providing a unified REST API for the deployment, scaling, and lifecycle management of containerized applications within OmniCloud.

## Features

- **Consensus Cluster**: Implements Raft consensus algorithm to ensure consistent platform state
- **Service Health Management**: Monitors and maintains the health of OmniCloud internal services
- **Application Management**: Create, update, delete, and monitor applications
- **Release Management**: Upload, track, and deploy application releases
- **Instance Monitoring**: View metrics, logs, and details for application instances
- **Cluster Setup**: Easy configuration for running multiple orchestration nodes
- **RESTful API**: Comprehensive API for integration with other OmniCloud components and external tools

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/app/<app_id>/info` | Get detailed information about an application including metrics, parent org, description, and version data |
| POST | `/app/<app_id>/release/<version_id>/upload` | Upload a new application release, creating a build job that automatically adds a release when complete |
| GET | `/app/<app_id>/metrics` | Retrieve lightweight metrics for an application (designed for frequently updating UIs) |
| POST | `/app/create` | Create a new application with the specified configuration |
| GET | `/app/<app_id>/instances` | Get a paginated list of instances with basic metrics |
| GET | `/app/<app_id>/instances/<id>/metrics` | Retrieve detailed metrics for a specific instance |
| GET | `/app/<app_id>/instances/<id>/logs` | Get paginated logs for a specific instance |
| GET | `/app/<app_id>/instances/<id>/details` | Get detailed information about a specific instance |
| PATCH | `/app/<app_id>` | Edit an application's details and/or configuration |
| DELETE | `/app/<app_id>` | Delete an application and all associated data (instances, logs, builds, etc.) |

## Getting Started

### Prerequisites

- Rust 1.82.0 or later
- MySQL database
- Docker (for containerized deployment)

### Configuration

Configuration is managed through a `config.json` file in the root directory:

```json
{
    "port": 8002,
    "address": "http://localhost",
    "highlight_sql": true,
    "instances": [
        {
            "port": 8000,
            "address": "http://localhost"
        },
        {
            "port": 8001,
            "address": "http://localhost"
        },
        {
            "port": 8002,
            "address": "http://localhost"
        },
        {
            "port": 8003,
            "address": "http://localhost"
        }
    ]
}
```

### Installation

#### From Source

1. Clone the repository:
   ```
   git clone https://github.com/OmniCloudOrg/OmniOrchestrator
   cd OmniOrchestrator
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

#### Using Docker

1. Build the Docker image:
   ```
   docker build -t omni-orchestrator .
   ```

2. Run the container:
   ```
   docker run -p 3000:3000 omni-orchestrator
   ```

#### Using Docker Compose

1. Start all services:
   ```
   docker compose up --build
   ```

### Consensus Clustering

Omni Orchestrator operates as a Raft consensus cluster, maintaining consistent state across all OmniCloud services while providing high availability and resilience.

#### On Linux/macOS:

Run the `start.sh` script to initialize a Raft consensus cluster based on your `config.json` configuration:
```
chmod +x start.sh
./start.sh
```

#### On Windows:

Run the `start.bat` script to initialize a Raft consensus cluster:
```
start.bat
```

The consensus algorithm automatically elects a leader node, synchronizes state across the cluster, and handles node failures while maintaining service continuity.

## Development

### Cross-Platform Building

For Linux targets, use the `build-linux.sh` script:
```
chmod +x build-linux.sh
./build-linux.sh
```

This will create builds for various architectures including:
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu
- And many more

### Dev Environment Setup

To set up a complete development environment:
```
chmod +x dev-setup.sh
./dev-setup.sh
```

This installs VirtualBox, AWS CLI, GCP CLI, govc CLI for vSphere, and OpenStack CLI.

## Database

The application requires a MySQL database. When using Docker Compose, a MySQL container is automatically created with the following configuration:

- Database name: `omni`
- Username: `root`
- Password: `root`
- Port: `4001` (mapped to container's 3306)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Architecture

Omni Orchestrator acts as a central gateway API to the OmniCloud platform, utilizing a Raft consensus protocol to:

1. **Coordinate Services**: Ensure all internal services operate in harmony
2. **Maintain State**: Preserve consistent state across the distributed system
3. **Ensure Availability**: Provide failure recovery and high availability
4. **Load Balance**: Distribute workloads across service instances efficiently
5. **Manage Deployments**: Orchestrate rolling updates and service deployments

The Raft consensus ensures that even in the face of node failures, network partitions, or other disruptions, the platform remains operational and maintains data consistency.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
