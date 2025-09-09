#!/bin/bash
openapi-generator generate -i https://vpn.ctf.agin.rocks/swagger/v1/openapiv2.json -g rust -o headscale --skip-validate-spec
echo "#![allow(warnings)]" > temp.rs && cat headscale/src/lib.rs >> temp.rs && mv temp.rs headscale/src/lib.rs
