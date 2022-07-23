# Divoom
![Divoom](https://raw.githubusercontent.com/r12f/divoom/main/assets/Logo.png)

Rust Library for controlling divoom devices that support REST APIs, such as pixoo-64 (and from how divoom's api/doc organizes, maybe more in the future).

[![Crates.io](https://img.shields.io/crates/v/divoom)](https://crates.io/crates/divoom)
[![Documentation](https://docs.rs/divoom/badge.svg)](https://docs.rs/divoom/)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Build Status](https://riff.visualstudio.com/divoom/_apis/build/status/r12f.divoom?branchName=main)](https://riff.visualstudio.com/divoom/_build/latest?definitionId=7&branchName=main)

```rust
// Get current channel
use divoom::*;

println!(
    "{}",
    PixooClient::new("192.168.0.123").get_current_channel().await?
);

// Output: clock
```

We have a command line tool as well to help people use and serve as demo! For more docs, feel free to check it here: <https://github.com/r12f/divoom/blob/main/divoom_cli/README.md>.

```bash
# Check current channel
> divoom-cli 192.168.0.123 channel get
---
clock
```

## How to use

The library contains 2 major parts:

- Divoom service APIs, that is used for talking to Divoom's backend service for device discovery etc.
- Pixoo device APIs, that is used for talking to a specific device via its REST APIs.

### Divoom service APIs

To discover all devices in your LAN, we can use the `get_same_lan_devices` API to get all devices from Divoom's backend service.

```rust
use divoom::*;

let divoom = DivoomServiceClient::new();
let devices = divoom.get_same_lan_devices().await?;
devices.iter().for_each(|x| println!("{:?}", x));
```

This will output:

```text
DivoomDeviceInfo { device_name: "Pixoo", device_id: 300000001, device_private_ip: "192.168.0.123" }
```

### Pixoo device APIs

Once we get the Device Address, we can use it to create a pixoo client and start talking to it:

```rust
use divoom::*;

let pixoo = PixooClient::new("192.168.0.123");
let result = pixoo.get_current_channel().await?;
println!("{:?}", result);
```

This will output:
```text
Clock
```

Currently, we have these APIs supported:

- Channel APIs
    - [x] Select channel
    - [x] Get current channel
    - [x] Select clock
    - [x] Get selected clock info
    - [x] Select cloud channel
    - [x] Select visualizer
    - [x] Select custom page
- System/Device APIs
    - [x] Get device settings
    - [x] Get device time
    - [x] Set device brightness
    - [x] Set device time
    - [x] Set device high light mode
    - [x] Set device hour mode
    - [x] Set device mirror mode
    - [x] Set device rotation angle
    - [x] Set device screen power state
    - [x] Set device temperature unit
    - [x] Set device time zone
    - [x] Set device weather area
    - [x] Set device white balance
- Tools APIs
    - [x] Set countdown tool
    - [x] Set noise tool
    - [x] Set scoreboard tool
    - [x] Set stopwatch tool
- Animation APIs
    - [x] Play gif from file
    - [x] Get next animation id
    - [x] Reset next animation id
    - [x] Send image animation
    - [x] Send text animation
    - [x] Clear all text area
    - [x] Play buzzer
- Batch APIs
    - [X] Batching commands
    - [X] Execute commands from url

#### Image Animation

Devices like Pixoo-64 supports play GIF file from file or even Internet directly, all we need is to specify a URL as below:

```rust
use divoom::*;

let pixoo = PixooClient::new("192.168.0.123");
pixoo.play_gif_file(DivoomFileAnimationSourceType::Url, "<Some URL goes here>").await?;
```

However, this device API is not quite stable (by 07/2022) and the most reliable way to play a GIF is to create an animation and draw
all the GIF frames into it one by one. To help with this process, we created a resource loader and an animation builder to help.

```rust
use divoom::*;

// Load the resource.
let frames = DivoomAnimationResourceLoader::gif("test_data/animation_builder_tests/logo-16-rotate-4-frames.gif").unwrap();

// Build animation with 16 pixel canvas and 100ms frame play speed.
let builder = DivoomAnimationBuilder::new(16, Duration::from_millis(100)).unwrap();
let animation = builder.draw_frames(&frames, 0).build();

// Send to device here.
let pixoo = PixooClient::new("192.168.0.123");
pixoo.send_image_animation(animation).await
```

Or even simpler:

```rust
use divoom::*;
let pixoo = PixooClient::new("192.168.0.123");
pixoo.send_gif_as_animation(16, Duration::from_millis(100), "test_data/animation_builder_tests/logo-16-rotate-4-frames.gif").await
```

For more on how to use it, feel free to check our doc here: <https://docs.rs/divoom/latest/divoom/struct.DivoomAnimationBuilder.html>.

And if you don't want this animation builder, we can exclude it by specifying the features with:

```toml
[dependencies]
divoom = { version = "0.1", features = [] }
```

#### Text Animation

To create a text animation, we can use `DivoomTextAnimation` structure and `send_text_animation` API to help us:

```rust
use divoom::*;

let pixoo = PixooClient::new("192.168.0.123");
let animation = DivoomTextAnimation::default(); 
animation.text_string = "Foo".to_string();
pixoo.send_text_animation(animation).await?;
```

#### Command batching

In certain cases, we might want to run a lot of commands at the same time, such as initialize the settings. Pixoo devices supports batching all commands into a single request, but with only 1 single result being returned for indicating if everything succeeded or not.

Here is an example that we batch executed multiple commands to update the device settings:

```rust
use divoom::*;
let pixoo = PixooClient::new("192.168.0.123");
pixoo.start_batch()
  .set_device_rotation_angle(DivoomDeviceRotationAngle::Rotate90)
  .set_device_mirror_mode(DivoomDeviceMirrorMode::On)
  .set_device_brightness(30)
  .execute().await.expect("Request should succeed.");
```

#### Sending raw requests

In case new API is released and we haven't support it yet, or we need to do some experimental things by sending the raw payload, we can use the following API to send raw request directly, which works for both single request and batch mode.

Single request mode:

```rust
use divoom::*;

let pixoo = PixooClient::new("192.168.0.123");
pixoo.send_raw_request("{ \"Command\": \"Device/SetHighLightMode\", \"Mode\": 0 }").await?.expect("Request should succeed.");
```

Batch mode:

```rust
use divoom::*;
let pixoo = PixooClient::new("192.168.0.123");
pixoo.start_batch()
  .send_raw_request("{ \"Command\": \"Device/SetHighLightMode\", \"Mode\": 0 }".into())
  .execute_with_raw_response().await.expect("Request should succeed.");
```

## Debugging

The debug logs are logged at debug level. Once we set the log level to debug, we will be able to start see it:

```rust
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
```

Or we can use `RUST_LOG` environment variable to change the level and enable the logs:

With the command tool (covered below soon), on windows:

```powershell
> $env:RUST_LOG="debug"; .\divoom-cli.exe 192.168.0.123 channel get
```

And on linux:

```bash
RUST_LOG=debug ./divoom-cli.exe 192.168.0.123 channel get
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

To revert it back:
```powershell
> $env:RUST_LOG="warn"; .\divoom-cli.exe 192.168.0.123 channel get
---
customPage
```

## License
Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>
