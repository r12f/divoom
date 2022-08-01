# divoom-gateway
![Divoom](https://raw.githubusercontent.com/r12f/divoom/main/assets/Logo.png)

A REST API gateway with swagger UI provided that wraps divoom HTTP APIs for controlling divoom devices, like pixoo (and from how divoom's api/doc organizes, maybe more http supported devices in the future).

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Build Status](https://riff.visualstudio.com/divoom/_apis/build/status/r12f.divoom?branchName=main)](https://riff.visualstudio.com/divoom/_build/latest?definitionId=7&branchName=main)

| Release | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|:---:|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Crates.io | [![Crates.io](https://img.shields.io/crates/v/divoom-gateway?color=blue&style=flat-square&label=cargo%20install%20divoom-gateway)](https://crates.io/crates/divoom-gateway)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Install | [![winget](https://img.shields.io/static/v1?style=flat-square&label=winget%20install%20DivoomGateway&message=winget&color=blue)](https://github.com/microsoft/winget-pkgs/tree/master/manifests/r/r12f/DivoomGateway)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Nuget<br/>packages | [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.windows.x86?style=flat-square&color=green&label=windows.x86)](https://www.nuget.org/packages/divoom-gateway.windows.x86/) [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.windows.x64?style=flat-square&color=green&label=windows.x64)](https://www.nuget.org/packages/divoom-gateway.windows.x64/) [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.windows.arm64?style=flat-square&color=green&label=windows.arm64)](https://www.nuget.org/packages/divoom-gateway.windows.arm64/) <br/> [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.linux.x86?style=flat-square&color=green&label=linux.x86)](https://www.nuget.org/packages/divoom-gateway.linux.x86/) [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.linux.x64?style=flat-square&color=green&label=linux.x64)](https://www.nuget.org/packages/divoom-gateway.linux.x64/) [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.linux.arm?style=flat-square&color=green&label=linux.arm)](https://www.nuget.org/packages/divoom-gateway.linux.arm/) [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.linux.arm64?style=flat-square&color=green&label=linux.arm64)](https://www.nuget.org/packages/divoom-gateway.linux.arm64/) <br/> [![Nuget](https://img.shields.io/nuget/v/divoom-gateway.linux.arm64?style=flat-square&color=green&label=macos.x64)](https://www.nuget.org/packages/divoom-gateway.macos.x64/) |

```bash
> divoom-gateway 192.168.0.123
Starting divoom gateway on: http://127.0.0.1:20821 for device 192.168.0.123.
Please open your browser with URL: http://127.0.0.1:20821 and happy divooming!

# Now, we can open http://127.0.0.1:20821 in browser! Happy Divooming!
```

## How to install

### via Cargo
```bash
cargo install divoom-gateway
```

### via winget
```powershell
winget install DivoomGateway
```

### via scoop
Since we haven't reached the criteria for Main bucket, we need to use our own bucket at this moment.

```powershell
# Add our scoop bucket for the first time.
scoop bucket add r12f https://github.com/r12f/scoop-bucket

# Install
scoop install divoom-gateway
```

## How to use

To start the gateway, we need 3 steps:

### 1. Find the IP address of your device.

There are multiple ways to discover the address:

1. Get the address in divoom app.
   ![image](https://user-images.githubusercontent.com/1533278/182060485-09cd2481-7031-4121-b21f-a5c0d6476069.png)

2. Or, try our [divoom-cli command line](https://github.com/r12f/divoom/tree/main/divoom_cli) tool and discover all devices.

   ```bash
   > .\divoom-cli.exe discover
   - device-name: Pixoo
     device-id: 300000001
     device-private-ip: 192.168.0.123
   ```

### 2. Start the gateway with device address assigned

```bash
> divoom-gateway 192.168.0.123
Starting divoom gateway on: http://127.0.0.1:20821 for device 192.168.0.123.
Please open your browser with URL: http://127.0.0.1:20821 and happy divooming!
```

If we are seeing the following error when launching the gateway, it means the default port is taken and we need to use another port:

```bash
Error: Os { code: 10048, kind: AddrInUse, message: "Only one usage of each socket address (protocol/network address/port) is normally permitted." }
```

We can use the below options to specify IP and port when needed:

```bash
# divoom-gateway <divoom-device-ip> -s <our-machine-ip> -p <port>
# In this case our machine IP is 192.168.0.234, and we are opening 20822 to connect to device 192.168.0.123:
> divoom-gateway 192.168.0.123 -s 192.168.0.234 -p 20822
Starting divoom gateway on: http://192.168.0.151:20822 for device 192.168.0.164.
Please open your browser with URL: http://192.168.0.151:20822 and happy divooming!
```

### 3. Open browser with URL, that's it!

![image](https://user-images.githubusercontent.com/1533278/182061620-2a0724cf-a153-4dc5-8330-63a1a49b5eb0.png)

## Supported commands

Currently, we support all commands that documented in divoom's public API doc. For details, please check it [here](https://github.com/r12f/divoom#pixoo-device-apis).

### Play GIF animation

Pixoo devices provided [an API to play GIF file by providing a file location](http://doc.divoom-gz.com/web/#/12?page_id=195), and we wrapped it up and
provided an API on `/api/animation/play-gif`.

![image](https://user-images.githubusercontent.com/1533278/182063159-5851d354-7305-41cd-9efe-e395b1cff91a.png)

However, this API is very restricted on the image size and not very stable. It could end up with crashing your device. Hence, we added another API
called `/api/animation/send-gif`, which allow us to upload a GIF file and generate an animation to play, which is much more stable.

![image](https://user-images.githubusercontent.com/1533278/182063297-ab7cebb6-1a87-42bb-a8bc-d4c63982c7fd.png)

### Play text animation

Once we have used the `/api/animation/send-gif` command to play any animation, we can start use text animation APIs, otherwise these APIs will be
no-op'ed by the device.

![image](https://user-images.githubusercontent.com/1533278/182063848-e3ac6409-f3f8-4228-932f-d661f782e16d.png)


## More help

We can find more info in the command help like below.

```bash
> divoom-gateway --help
divoom-gateway 0.0.1
r12f <r12f.code@gmail.com>
A REST API gateway with swagger UI provided that wraps divoom HTTP APIs for controlling divoom
devices, like pixoo.

USAGE:
    divoom-gateway.exe [OPTIONS] <DEVICE_ADDRESS>

ARGS:
    <DEVICE_ADDRESS>    Device address.

OPTIONS:
    -h, --help                       Print help information
    -p, --port <SERVER_PORT>         Server port. [default: 20821]
    -s, --server <SERVER_ADDRESS>    Server address. [default: 127.0.0.1]
    -V, --version                    Print version information
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

```bash
Starting divoom gateway on: http://127.0.0.1:20821 for device 192.168.0.123.
Please open your browser with URL: http://127.0.0.1:20821 and happy divooming!
[2022-08-01T02:59:40Z INFO  poem::server] listening addr=socket://127.0.0.1:20821
[2022-08-01T02:59:40Z INFO  poem::server] server started
[2022-08-01T02:59:42Z DEBUG hyper::proto::h1::io] parsed 17 headers
[2022-08-01T02:59:42Z DEBUG hyper::proto::h1::conn] incoming body is content-length (230 bytes)
[2022-08-01T02:59:42Z DEBUG hyper::proto::h1::conn] incoming body completed
[2022-08-01T02:59:42Z DEBUG divoom::clients::common::divoom_rest_client] Sending request: Url = "http://192.168.0.123/post", Body = "{"Command":"Draw/SendHttpText","TextId":0,"x":0,"y":0,"dir":1,"font":0,"TextWidth":0,"speed":100,"TextString":"The gray fox jumped over the lazy dog","color":"#000000","align":2}", Timeout = 2s
[2022-08-01T02:59:42Z DEBUG reqwest::connect] starting new connection: http://192.168.0.123/
[2022-08-01T02:59:42Z DEBUG hyper::client::connect::http] connecting to 192.168.0.123:80
[2022-08-01T02:59:42Z DEBUG hyper::client::connect::http] connected to 192.168.0.123:80
...
```

To revert it back, we can use the same way to set the `RUST_LOG` to `warn` level:

```powershell
> $env:RUST_LOG="warn"
```

## API && SDK

If you are interested in the APIs that this tool is calling and the rust SDK that it uses, please check it here: <https://github.com/r12f/divoom/blob/main/README.md>.

## License
Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>
