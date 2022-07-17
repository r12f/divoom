# divoom-cli
![Divoom](https://raw.githubusercontent.com/r12f/divoom/main/assets/Logo.png)

Command line tool built on top of divoom APIs for controlling divoom devices, like pixoo (and from how divoom's api/doc organizes, maybe more in the future).

[![Crates.io](https://img.shields.io/crates/v/divoom)](https://crates.io/crates/divoom-cli)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

```rust
# Check current channel
> divoom-cli 192.168.0.123 channel get
clock

# Set channel to clock with id 100
> divoom-cli 192.168.0.164 channel set-clock 100

# Get clock channel info
> divoom-cli 192.168.0.164 channel get-clock
---
clock-id: 100
brightness: 100
```

## Installation

### Cargo
```bash
cargo install divoom-cli
```

## How to use

It is straight forward to use the command line tool. Usually, we do it in 2 steps:

- Use device discovery API to find all devices in the LAN that we are in.
- Use device APIs to control the devices. No login/authentication is needed in the process.

### Discover devices in the same LAN

```bash
> divoom-cli discover
---
- device-name: Pixoo
  device-id: 300000001
  device-private-ip: 192.168.0.123
```

### Use Devices APIs to control the device

All device APIs are having the same format:

```bash
divoom-cli <device-address> <api-category> <api> [parameters]
```

So we can run the commands like below:

```bash
# Check current channel
> divoom-cli 192.168.0.123 channel get
Clock

# Check current clock
> divoom-cli 192.168.0.123 channel get-clock
---
clock_id: 100
brightness: 67

# Set channel to clock with id 100
> divoom-cli 192.168.0.164 channel set-clock 100

# Create a text animation
> divoom-cli 192.168.0.123 animation text set 1 "The gray fox jumped over the lazy dog"

# Play a gif from Internet
> divoom-cli 192.168.0.123 animation gif play --url https://www.gifandgif.eu/animated_gif/Planets/Animated%20Gif%20Planets%20(16).GIF

# Send a raw request
#
# NOTICE: the double quotes in json string passed into the program needs to escaped with '\',
# otherwise, rust runtime (not structopt) will eat them before reaching main function, even we 
# pass the whole string as a string.
> divoom-cli 192.168.0.164 raw '{\"Command\": \"Device/SetHighLightMode\", \"Mode\": 0}'
```

### Paramter/Output format

By default, divoom-cli uses yaml as output format with all field names in `kebab-case`. And beside yaml, we support json format too.

To specify the output format, we can use `-o` parameter:

```bash
> divoom-cli -o json 192.168.0.164 channel get-clock
{"clock-id":100,"brightness":67}

> divoom-cli -o yaml 192.168.0.164 channel get-clock
---
clock-id: 100
brightness: 67
```

For values, divoom-cli always expect `camelCase` to be used, both in parameter and output.

### More help

We can find more info in the commmand help like below.

```bash
> divoom-cli
divoom-cli 0.0.1
r12f
https://github.com/r12f/divoom

USAGE:
    divoom-cli.exe [OPTIONS] [device-address] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output format. [default: yaml]

ARGS:
    <device-address>    Device Address. Required when using device APIs, such as "channel get".

SUBCOMMANDS:
    animation    Animation related APIs
    batch        Batch related APIs
    channel      Channel related APIs
    discover     Discover divoom devices by calling into divoom service API
    help         Prints this message or the help of the given subcommand(s)
    raw          Sending raw request
    system       System/device related APIs
    tool         APIs to launch some tools
```

## Debugging

To debug and see the logs and the raw request that we send, we can use `RUST_LOG` environment variable to change the logging level to `debug` to enable the logs:

On windows with powershell:

```powershell
$env:RUST_LOG="debug"; divoom-cli 192.168.0.123 channel get
```

On Windows with cmd:

```cmd
set RUST_LOG=debug && divoom-cli 192.168.0.164 channel get
```

And on linux:

```bash
RUST_LOG=debug divoom-cli 192.168.0.123 channel get
```

Then we will see the output log like below:

```text
[2022-07-10T00:33:50Z DEBUG divoom::clients::common::divoom_rest_client] Sending request: Url = "http://192.168.0.123/post", Body = "{"Command":"Channel/GetIndex"}"
[2022-07-10T00:33:50Z DEBUG reqwest::connect] starting new connection: http://192.168.0.123/
[2022-07-10T00:33:50Z DEBUG hyper::client::connect::http] connecting to 192.168.0.123:80
[2022-07-10T00:33:50Z DEBUG hyper::client::connect::http] connected to 192.168.0.123:80
[2022-07-10T00:33:50Z DEBUG hyper::proto::h1::io] flushed 107 bytes
[2022-07-10T00:33:50Z DEBUG hyper::proto::h1::io] parsed 2 headers
[2022-07-10T00:33:50Z DEBUG hyper::proto::h1::conn] incoming body is chunked encoding
[2022-07-10T00:33:50Z DEBUG hyper::proto::h1::decode] incoming chunked header: 0x22 (34 bytes)
[2022-07-10T00:33:50Z DEBUG reqwest::async_impl::client] response '200 OK' for http://192.168.0.123/post
[2022-07-10T00:33:50Z DEBUG divoom::clients::common::divoom_rest_client] Response header received: StatusCode = 200
[2022-07-10T00:33:50Z DEBUG hyper::proto::h1::conn] incoming body completed
[2022-07-10T00:33:50Z DEBUG hyper::client::pool] pooling idle connection for ("http", 192.168.0.123)
[2022-07-10T00:33:50Z DEBUG divoom::clients::common::divoom_rest_client] Response received: Body = "{"error_code": 0, "SelectIndex":3}"
---
customPage
```

To revert it back, we can use the same way to set the `RUST_LOG` to `warn` level:

```powershell
> $env:RUST_LOG="warn"; divoom-cli 192.168.0.123 channel get
---
customPage
```

## API && SDK

If you are interested in the APIs that this tool is calling and the rust SDK that it uses, please check it here: <https://github.com/r12f/divoom/blob/main/README.md>.

## License
Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>