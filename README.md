# CTFILT 2

`ctfilt` is a comprehensive platform for creating and managing Capture The Flag (CTF) challenges and courses. It consists of a central server, various supporting services, and command-line tools for administration. The project is built primarily in Rust and leverages technologies like Kubernetes and OpenAPI.

## Project Overview

This project provides a complete infrastructure for hosting CTF challenges and educational cybersecurity courses. The platform allows for the creation, deployment, and management of containerized challenges while providing a user-friendly interface for both administrators and participants.

## Project Structure

The `ctfilt` project is a monorepo containing several Rust workspaces and other configuration files.

### Subfolders

Here's a detailed breakdown of the subfolders and their roles within the project:

#### [`api_client/`](api_client)

A Rust client library for interacting with the main `ctfilt` server's API. It's automatically generated using OpenAPI Generator based on the server's API schema.

- **Purpose**: Provides a type-safe way to interact with the server's REST API from other Rust applications.
- **Key Features**:
  - Full coverage of all server API endpoints
  - Generated documentation for all models and endpoints
  - Type-safe request and response handling

#### [`ctfsync/`](ctfsync)

A command-line interface (CLI) tool used to synchronize CTF challenges and courses with the `ctfilt` server.

- **Purpose**: Enables content creators and administrators to manage platform content efficiently.
- **Key Dependencies**:
  - Uses the `api_client` for server communication
  - Integrates with `manifests` for schema validation
  - Includes Git integration for version control

#### [`exterminator/`](exterminator)

A focused Rust utility designed to interact with the Kubernetes API for pod management.

- **Purpose**: Provides an abstraction layer for other components to interact with Kubernetes.
- **Key Features**:
  - Kubernetes API integration
  - Error handling for Kubernetes operations

#### [`headscale/`](headscale)

Contains a generated Rust client for the Headscale API. Headscale is an open-source implementation of the Tailscale coordination server.

- **Purpose**: Manages the private networking layer of the CTF environment.
- **Key Features**:
  - Node management (listing, creating, deleting)
  - User management
  - Pre-auth key generation
  - API key management

#### [`manifests/`](manifests)

Defines the core data structures (or "manifests") for the project, such as what constitutes a `Challenge`, `Course`, or `Lesson`.

- **Purpose**: Ensures consistency in data structures across the platform.
- **Key Features**:
  - Schema definitions with optional validation
  - Support for serialization/deserialization
  - Integration with utoipa for API documentation

#### [`pod_assassin/`](pod_assassin)

A Kubernetes controller that watches for pods with specific labels and automatically deletes them when they expire.

- **Purpose**: Manages the lifecycle of ephemeral challenge environments.
- **Key Features**:
  - Label-based pod monitoring
  - Configurable expiration handling
  - Optional deletion of related resources
  - Supports both in-cluster and local kubeconfig modes

#### [`server/`](server)

The heart of the `ctfilt` platform. This is an Axum-based web server that exposes the main API.

- **Purpose**: Provides the central API and business logic for the platform.
- **Key Features**:
  - Course and challenge management
  - User authentication and authorization
  - Integration with Kubernetes for challenge deployment
  - MongoDB database integration
  - OpenAPI documentation

#### [`web/`](web)

Contains the frontend application for the `ctfilt` platform.

- **Purpose**: Provides the user-facing interface for the platform.
- **Key Features**:
  - Challenge interface for participants
  - Course navigation
  - Admin interface for content management

### Top-Level Files

- **`Caddyfile`**: Configuration for the Caddy web server, acting as a development reverse proxy for the application.
- **`generate-ctfilt-client.sh`/`generate-headscale-client.sh`**: Scripts for generating API clients.

## [Getting Started](#getting-started)

1. Ensure you have Rust and Kubernetes installed
2. Clone the repository
3. Build the project with `cargo build --release`
4. Configure the server using `server/config.toml`
5. Start the server with `cargo run -p server`


## License

Please refer to the license file for details on project licensing.
