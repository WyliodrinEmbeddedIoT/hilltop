default:
    just -l

esp32:
    python3 ./tester.py --client --job-description ../sample-jobs/esp32_hello/job.json --job-data ../sample-jobs/esp32_hello.zip

nrf52:
    python3 ./tester.py --client --job-description ../sample-jobs/nrf52_blinky/job.json --job-data ../sample-jobs/nrf52_blinky.zip

dummy:
    python3 ./tester.py --client --job-description ../sample-jobs/dummy/job.json --job-data ../sample-jobs/dummy.zip

samples:
    cd ../sample-jobs && just samples
