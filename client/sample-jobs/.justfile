default:
    just -l

sample-esp32_hello:
    cd esp32_hello && cargo clean
    rm -f ./esp32_hello.zip
    cd esp32_hello && zip -r ../esp32_hello.zip .
    echo "ESP32 Hello Done"

sample-nrf52_blinky:
    cd nrf52_blinky && cargo clean
    rm -f ./nrf52_blinky.zip
    cd nrf52_blinky && zip -r ../nrf52_blinky.zip .
    echo "NRF52 Blinky Done"

sample-dummy:
    cd dummy
    rm -f ./dummy.zip
    cd dummy && zip -r ../dummy.zip .
    echo "Dummy Done"

samples: sample-esp32_hello sample-nrf52_blinky sample-dummy
    echo "Sample jobs created successfully."