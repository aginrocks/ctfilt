ctfilt-client:
    openapi-generator generate -i http://localhost:8080/apidoc/openapi.json -g rust -o api_client --skip-validate-spec
    echo "#![allow(warnings)]" > temp.rs && cat api_client/src/lib.rs >> temp.rs && mv temp.rs api_client/src/lib.rs

headscale-client:
    openapi-generator generate -i https://vpn.ctf.agin.rocks/swagger/v1/openapiv2.json -g rust -o headscale --skip-validate-spec
    echo "#![allow(warnings)]" > temp.rs && cat headscale/src/lib.rs >> temp.rs && mv temp.rs headscale/src/lib.rs
