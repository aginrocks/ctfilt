# Headscale Bridge

A server that bridges Headscale to the rest of the systems. It polls Headscale for changes and writes the data to Valkey.

> [!WARNING]
> High Availability (HA) is pointless in this case. The entire purpose of this server is to poll Headscale from one place.

*This server is a temporary solution until a better way can be found to stream data from Headscale*

## Configuration

Pod Assassin uses TOML configuration files with support for environment variable overrides.
