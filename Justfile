# Generate Rust API client
rust-api:
    openapi-generator generate -i http://localhost:8080/apidoc/openapi.json -g rust -o api_client --skip-validate-spec
    echo "#![allow(warnings)]" > temp.rs && cat api_client/src/lib.rs >> temp.rs && mv temp.rs api_client/src/lib.rs

# Generate TypeScript API client
ts-api:
    cd web && pnpm typegen

# Generate TypeScript types for WebSocket events
ts-types:
    cd server && cargo test export_bindings

typegen: rust-api ts-api ts-types

# Generate Rust client for headscale API
headscale-api:
    openapi-generator generate -i https://vpn.ctf.agin.rocks/swagger/v1/openapiv2.json -g rust -o headscale --skip-validate-spec
    echo "#![allow(warnings)]" > temp.rs && cat headscale/src/lib.rs >> temp.rs && mv temp.rs headscale/src/lib.rs

export BRANCH := `git rev-parse --abbrev-ref HEAD`
export COMMIT := `git rev-parse --short HEAD`

build-server:
    docker buildx bake server --push

build-web:
    docker buildx bake web --push
