# Pod Assassin

A Kubernetes controller that watches and deletes expired pods based on custom labels.

## Overview

Pod Assassin is a Rust-based Kubernetes controller that monitors pods with specific labels and deletes them if they expire. It's designed to work both in-cluster and with local kubeconfig files.

> [!WARNING]
> High Availability (HA) is not currently supported. Only run **one instance** of Pod Assassin at a time to avoid conflicts and race conditions.

## Features

- **Label-based pod watching**: Monitors pods with configurable label selectors
- **Flexible configuration**: TOML-based configuration with environment variable overrides
- **Related resource management**: Optionally deletes related resources when a pod is deleted

## Configuration

Pod Assassin uses TOML configuration files with support for environment variable overrides.

### Configuration File

Create a `config.toml` file in your working directory:

```toml
[kubernetes]
mode = "kubeconfig"  # or "incluster"

[labels]
prefix = "pod-assassin.agin.rocks"
delete_related = true
```

### Configuration Options

#### `[kubernetes]`
- `mode`: Authentication mode
  - `"kubeconfig"`: Use local kubeconfig file (default for development)
  - `"incluster"`: Use in-cluster service account (for pod deployments)

#### `[labels]`
- `prefix`: Label prefix for pod selection (default: `"pod-assassin.agin.rocks"`)
- `delete_related`: Whether to delete related resources (default: `true`)

### Environment Variables

Configuration can be overridden using environment variables with the `ASSASSIN__` prefix:

```bash
export ASSASSIN__KUBERNETES_MODE=incluster
export ASSASSIN__LABELS_PREFIX=my-custom-prefix
export ASSASSIN__LABELS_DELETE_RELATED=false
```

### Logging Configuration

Set the `RUST_LOG` environment variable to control logging levels:

```bash
export RUST_LOG=info                    # Default level
export RUST_LOG=debug                   # Debug level
export RUST_LOG=pod_assassin=trace      # Trace level for this crate only
```
