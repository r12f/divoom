# divoom-cli
![Divoom](https://raw.githubusercontent.com/r12f/divoom/main/assets/Logo.png)

Command line tool built on top of divoom APIs for controlling divoom devices, like pixoo (and from how divoom's api/doc organizes, maybe more in the future).

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Build Status](https://riff.visualstudio.com/divoom/_apis/build/status/r12f.divoom?branchName=main)](https://riff.visualstudio.com/divoom/_build/latest?definitionId=7&branchName=main)

| Release | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|:---:|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Crates.io | [![Crates.io](https://img.shields.io/crates/v/divoom-cli?color=blue&style=flat-square&label=cargo%20install%20divoom-cli)](https://crates.io/crates/divoom-cli)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Install | [![winget](https://img.shields.io/static/v1?style=flat-square&label=winget%20install%20DivoomCli&message=winget&color=blue)](https://github.com/microsoft/winget-pkgs/tree/master/manifests/r/r12f/DivoomCli)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Nuget<br/>packages | [![Nuget](https://img.shields.io/nuget/v/divoom-cli.windows.x86?style=flat-square&color=green&label=windows.x86)](https://www.nuget.org/packages/divoom-cli.windows.x86/) [![Nuget](https://img.shields.io/nuget/v/divoom-cli.windows.x64?style=flat-square&color=green&label=windows.x64)](https://www.nuget.org/packages/divoom-cli.windows.x64/) [![Nuget](https://img.shields.io/nuget/v/divoom-cli.windows.arm64?style=flat-square&color=green&label=windows.arm64)](https://www.nuget.org/packages/divoom-cli.windows.arm64/) <br/> [![Nuget](https://img.shields.io/nuget/v/divoom-cli.linux.x86?style=flat-square&color=green&label=linux.x86)](https://www.nuget.org/packages/divoom-cli.linux.x86/) [![Nuget](https://img.shields.io/nuget/v/divoom-cli.linux.x64?style=flat-square&color=green&label=linux.x64)](https://www.nuget.org/packages/divoom-cli.linux.x64/) [![Nuget](https://img.shields.io/nuget/v/divoom-cli.linux.arm?style=flat-square&color=green&label=linux.arm)](https://www.nuget.org/packages/divoom-cli.linux.arm/) [![Nuget](https://img.shields.io/nuget/v/divoom-cli.linux.arm64?style=flat-square&color=green&label=linux.arm64)](https://www.nuget.org/packages/divoom-cli.linux.arm64/) <br/> [![Nuget](https://img.shields.io/nuget/v/divoom-cli.linux.arm64?style=flat-square&color=green&label=macos.x64)](https://www.nuget.org/packages/divoom-cli.macos.x64/) |


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

## How to install

### via Cargo
```bash
cargo install divoom-cli
```

### via winget
```powershell
winget install DivoomCli
```

### via scoop
Since we haven't reached the [criteria for Scoop Main bucket](https://github.com/ScoopInstaller/Scoop/wiki/Criteria-for-including-apps-in-the-main-bucket), we need to use our own bucket at this moment.

```powershell
# Add our scoop bucket for the first time.
scoop bucket add r12f https://github.com/r12f/scoop-bucket

# Install
scoop install divoom-cli
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
---
clock

# Check current clock
> divoom-cli 192.168.0.123 channel get-clock
---
clock_id: 100
brightness: 67

# Set channel to clock with id 100
> divoom-cli 192.168.0.164 channel set-clock 100

# Play a gif from Internet by calling the API provided by Divoom Device.
# Please note that: this API can be unstable and only accepts GIF with 16x16, 32x32 and 64x64 image size.
> divoom-cli 192.168.0.123 animation gif play --url https://www.gifandgif.eu/animated_gif/Planets/Animated%20Gif%20Planets%20(16).GIF

# To help playing GIF in a more stable way, we can use the image animation API to craft an animation and draw the GIF
# frames into it and send to device to play, e.g.:
> divoom-cli 192.168.0.123 animation image send-gif "logo-16-rotate-4-frames.gif" 16 -s 100

# Create the same GIF animation as above, but with size stretched, rotation 30 degrees and opacity 0.5.
> divoom-cli 192.168.0.123 animation image send-gif "logo-16-rotate-4-frames.gif" 32 -s 100 -f stretch -o 0.5 -r 30

# Create a text animation
# Please note that: this API only works after we use "animation image send-gif" API to draw anything. This API call will be ignored, 
# when the device is showing other things, like clock or channel.
> divoom-cli 192.168.0.123 animation text set 1 "Hello world!"
> divoom-cli 192.168.0.123 animation text set 2 "The gray fox jumped over the lazy dog" -y 20

# Modify existing text animation. E.g. changing "Hello world!" above to "Hello Divoom!"
> divoom-cli 192.168.0.123 animation text set 1 "Hello Divoom!"

# Send a raw request
#
# NOTICE: the double quotes in json string passed into the program needs to escaped with '\',
# otherwise, rust runtime (not structopt) will eat them before reaching main function, even we 
# pass the whole string as a string.
> divoom-cli 192.168.0.164 raw '{\"Command\": \"Device/SetHighLightMode\", \"Mode\": 0}'
```

### Parameter/Output format

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

```bash
> divoom-cli 192.168.0.123 channel set customPage

> divoom-cli 192.168.0.123 channel get
---
customPage
```

### More help

We can find more info in the command help like below.

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

On Windows with powershell:

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