#!/bin/bash
openapi-generator generate -i http://localhost:8080/apidoc/openapi.json -g rust -o api_client --skip-validate-spec
echo "#![allow(warnings)]" > temp.rs && cat ctfilt_api/src/lib.rs >> temp.rs && mv temp.rs ctfilt_api/src/lib.rs
