# VCM Firmware

## Pre-requisites
See other documenation on how to setup Rust toolchain for embedded development

### Rust targets
To compile a rust application for an embedded target you first need to install the corresponding target architecture.

#### Main MCU
The main MCU is a STM32F4xx that require the `thumbv7em-none-eabihf` target installed:

```bash
rustup target add thumbv7em-none-eabihf
```

#### Monitoring MCU
The monitoring MCU is a STM32F0xx that retuire the `thumbv6m-none-eabi` target installed:

```bash
rustup target add thumbv6m-none-eabi
```