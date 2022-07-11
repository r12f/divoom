#!/usr/bin/env just --justfile

# Use powershell 7 on all platforms.
set shell := ["pwsh", "-NoLogo", "-Command"]

# Build environment settings
BUILD_TARGET := os_family() + "-" + (if os() != "macos" { "" } else { "macos-" }) + env_var_or_default("BUILD_TARGET", arch())
BUILD_TOOL_TARGET := if BUILD_TARGET == "windows-x86" {
    "i686-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-x86_64" {
    "x86_64-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-aarch64" {
    "aarch64-pc-windows-msvc"
  } else if BUILD_TARGET == "unix-x86" {
    "i686-unknown-linux-gnu"
  } else if BUILD_TARGET == "unix-x86_64" {
    "x86_64-unknown-linux-gnu"
  } else if BUILD_TARGET == "unix-arm" {
    "arm-unknown-linux-gnueabi"
  } else if BUILD_TARGET == "unix-aarch64" {
    "aarch64-unknown-linux-gnu"
  } else if BUILD_TARGET == "unix-macos-x86_64" {
    "x86_64-apple-darwin"
  } else {
    error("Unsupported platform")
  }

BUILD_PROFILE := env_var_or_default("BUILD_PROFILE", "dev")
BUILD_FLAVOR := if BUILD_PROFILE == "dev" { "debug" } else { "release" }
BUILD_OUTPUT_FOLDER := "target/" + BUILD_TOOL_TARGET + "/" + BUILD_FLAVOR
BIN_FILE_PATH_DIVOOM_CLI := BUILD_OUTPUT_FOLDER + "/divoom-cli.exe"

# Signing settings
BUILD_SIGNING_URL := env_var_or_default("BUILD_SIGNING_URL", "")
BUILD_SIGNING_VAULT_URL := env_var_or_default("BUILD_SIGNING_VAULT_URL", "")
BUILD_SIGNING_TENANT_ID := env_var_or_default("BUILD_SIGNING_TENANT_ID", "")
BUILD_SIGNING_CLIENT_ID := env_var_or_default("BUILD_SIGNING_CLIENT_ID", "")
BUILD_SIGNING_CLIENT_SECRET := env_var_or_default("BUILD_SIGNING_CLIENT_SECRET", "")
BUILD_SIGNING_CERT_NAME := env_var_or_default("BUILD_SIGNING_CERT_NAME", "")

# Publish
PUBLISH_DIR := "./publish"

#
# Default task:
#
default: format lint build test


#
# Init tasks: Installing build tools and etc
#
init-win:
    rustup default stable
    rustup target install {{BUILD_TOOL_TARGET}}

    # Install AzureSignTool
    # dotnet tool update is now the better (or more expected) way to install the tools. For details, please see the PR and issue below:
    # - https://github.com/dotnet/cli/pull/10205
    # - https://github.com/dotnet/sdk/issues/9500
    echo "Installing AzureSignTool."
    dotnet tool update --global azuresigntool

init-linux:
    #!/usr/bin/env bash

    # "sudo" needs to be added within this script, since running "sudo cargo" might results in cargo not found error.
    sudo apt update

    # Install GCC and required libs/tools
    echo "Installing build tools and required libs."
    sudo apt install -y build-essential libssl-dev p7zip-full

    case "{{BUILD_TOOL_TARGET}}" in
        "i686-unknown-linux-gnu")
            # For building x86 binary, we are using gcc-multilib.
            # This package is conflicting with other gcc-* packages, but we don't know any better package to use.
            # But sadly, this package is lacking of tools that we need to build ARM/ARM64, so we can only pick 1 to use - either support x86 or ARM/ARM64.
            sudo apt install -y gcc-multilib
            ;;
        "arm-unknown-linux-gnueabi")
            sudo apt install -y gcc-arm-linux-gnueabi binutils-arm-linux-gnueabi
            ;;
        "aarch64-unknown-linux-gnu")
            sudo apt install -y gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
            ;;
    esac

    # Install toolchains for cross builds
    echo "Installing rust target: {{BUILD_TOOL_TARGET}}"
    rustup default stable
    rustup target install {{BUILD_TOOL_TARGET}}

init-mac:
    echo "Installing build tools: binutils"
    brew install binutils


#
# Development tasks:
#
commit m: format lint build test
    git add .
    git commit -m "{{m}}"
    git push


#
# Format task:
#
format:
    cargo fmt -- --emit files


#
# Lint task:
#
lint:
    cargo clippy

lint-fix:
    cargo clippy --fix --allow-dirty


#
# Build / test tasks:
#
build:
    cargo build --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}

doc:
    cargo doc

test:
    cargo test --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}


#
# Sign task:
#
sign:
    AzureSignTool sign \
        -du "{{BUILD_SIGNING_URL}}" \
        -kvu "{{BUILD_SIGNING_VAULT_URL}}" \
        -kvt "{{BUILD_SIGNING_TENANT_ID}}" \
        -kvi "{{BUILD_SIGNING_CLIENT_ID}}" \
        -kvs "{{BUILD_SIGNING_CLIENT_SECRET}}" \
        -kvc "{{BUILD_SIGNING_CERT_NAME}}" \
        -v "{{BIN_FILE_PATH_DIVOOM_CLI}}"


#
# Install task:
#
install:
    cargo install --profile release --path ./divoom_cli

#
# Pack tasks:
#
pack:


#
# Publish tasks:
#
publish-dry package="divoom":
    cargo publish --dry-run -p {{package}}

    echo "Files in package:"
    cargo package --list -p {{package}}

publish package="divoom":
    cargo publish -p {{package}}