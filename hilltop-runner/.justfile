default: 
    just -l    

# Generate OpenAPI clients from YAML files in docker-engine-api
generate-api:
    #!/usr/bin/env sh
    for file in docker-engine-api/*.yaml; do
        filename=$(basename "${file}" .yaml)
        echo "Processing: ${filename}"; \
        openapi-generator-cli generate -i "${file}" -g rust -o "./api/api-${filename}"; \
        echo "Generated api-${filename}\n"
    done

    echo "All API files generated successfully!"
    
generate-broker-api:
    mkdir -p broker-api
    curl http://localhost:5288/swagger/v1/swagger.json -o broker-api/broker-api.yaml
    openapi-generator-cli generate -i broker-api/broker-api.yaml -g rust -o ./api/api-broker

run-esp32:
    rm -f ./config/runner.json
    cp ./config/runner.esp.json ./config/runner.json
    RUST_LOG=hilltop_runner=debug cargo run || true
    rm -f ./config/runner.json

run-nrf52:
    rm -f ./config/runner.json
    cp ./config/runner.nrf.json ./config/runner.json
    RUST_LOG=hilltop_runner=debug cargo run || true
    rm -f ./config/runner.json

run-both:
    rm -f ./config/runner.json
    cp ./config/runner.both.json ./config/runner.json
    RUST_LOG=hilltop_runner=debug cargo run || true
    rm -f ./config/runner.json

run-dummy:
    rm -f ./config/runner.json
    cp ./config/runner.dummy.json ./config/runner.json
    RUST_LOG=hilltop_runner=debug cargo run || true
    rm -f ./config/runner.json
