#!/usr/bin/env just --justfile

# Use powershell 7 on all platforms.
set shell := ["pwsh", "-NoLogo", "-Command"]

# Build environment settings
BUILD_OS := if os_family() == "windows" {
    "windows"
  } else if os() == "macos" {
    "macos"
  } else {
    "linux"
  }

BUILD_ARCH := if env_var_or_default("BUILD_ARCH", arch()) == "x86_64" {
    "x64"
  } else {
    env_var_or_default("BUILD_ARCH", arch())
  }

BUILD_TARGET := BUILD_OS + "-" + BUILD_ARCH

BUILD_TOOL_TARGET := if BUILD_TARGET == "windows-x86" {
    "i686-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-x64" {
    "x86_64-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-aarch64" {
    "aarch64-pc-windows-msvc"
  } else if BUILD_TARGET == "unix-x86" {
    "i686-unknown-linux-gnu"
  } else if BUILD_TARGET == "unix-x64" {
    "x86_64-unknown-linux-gnu"
  } else if BUILD_TARGET == "unix-arm" {
    "arm-unknown-linux-gnueabi"
  } else if BUILD_TARGET == "unix-aarch64" {
    "aarch64-unknown-linux-gnu"
  } else if BUILD_TARGET == "macos-x64" {
    "x86_64-apple-darwin"
  } else {
    error("Unsupported platform")
  }

BUILD_PROFILE := env_var_or_default("BUILD_PROFILE", "dev")
BUILD_FLAVOR := if BUILD_PROFILE == "dev" { "debug" } else { "release" }
BUILD_OUTPUT_FOLDER := "target/" + BUILD_TOOL_TARGET + "/" + BUILD_FLAVOR
BUILD_VERSION := trim(`gc ./build/version.txt | Select-String '\d+\.\d+\.\d+' | % { $_.Matches[0].Value }`)

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
    @echo "Installing AzureSignTool."
    dotnet tool update --global azuresigntool

init-linux:
    #!/usr/bin/env bash

    # "sudo" needs to be added within this script, since running "sudo cargo" might results in cargo not found error.
    sudo apt update

    # Install GCC and required libs/tools
    @echo "Installing build tools and required libs."
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
    @echo "Installing rust target: {{BUILD_TOOL_TARGET}}"
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
pack-prepare package="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{package}}") { Remove-Item -Path "{{PUBLISH_DIR}}/{{package}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{package}}" -Force | Out-Null

pack-bin package="divoom_cli": (pack-binary package) (pack-symbol package)

pack-binary package="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{package}}/bin") { Remove-Item -Path "{{PUBLISH_DIR}}/{{package}}/bin" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{package}}/bin" -Force | Out-Null

    $fileNames = @("{{replace(package, '_', '-')}}.exe", "{{replace(package, '_', '-')}}" ); \
    foreach ($fileName in $fileNames) { \
      $filePath = "{{BUILD_OUTPUT_FOLDER}}/$fileName"; if (Test-Path $filePath) { Write-Host "Copy binary file: $filePath"; Copy-Item -Path $filePath -Destination "{{PUBLISH_DIR}}/{{package}}/bin" } \
    }

pack-symbol package="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{package}}/symbols") { Remove-Item -Path "{{PUBLISH_DIR}}/{{package}}/symbols" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{package}}/symbols" -Force | Out-Null

    $fileNames = @("{{package}}.pdb", "{{package}}.debug" ); \
    foreach ($fileName in $fileNames) { \
      $filePath = "{{BUILD_OUTPUT_FOLDER}}/$fileName"; if (Test-Path $filePath) { Write-Host "Copy symbol file: $filePath"; Copy-Item -Path $filePath -Destination "{{PUBLISH_DIR}}/{{package}}/symbols" } \
    }

pack-binary-zip package="divoom_cli":
    if (-not (Test-Path "{{PUBLISH_DIR}}/{{package}}/zip")) { New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{package}}/zip" -Force | Out-Null }

    if ("{{BUILD_OS}}" -eq "windows") { \
        $packageName = "{{replace(package, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.zip"; \
        7z -tzip a "{{PUBLISH_DIR}}/{{package}}/zip/$packageName" "{{PUBLISH_DIR}}/{{package}}/bin/*"; \
    } else { \
        $packageName = "{{replace(package, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.tar"; \
        7z -ttar a "{{PUBLISH_DIR}}/{{package}}/zip/$packageName" "{{PUBLISH_DIR}}/{{package}}/bin/*"; \
        7z -tgzip a "{{PUBLISH_DIR}}/{{package}}/zip/$packageName.gz" "{{PUBLISH_DIR}}/{{package}}/zip/$packageName"; \
        Remove-Item "{{PUBLISH_DIR}}/{{package}}/zip/$packageName"; \
    }

pack-source:
    if (Test-Path "{{PUBLISH_DIR}}/temp/source") { Remove-Item -Path "{{PUBLISH_DIR}}/temp/source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/temp/source" -Force | Out-Null

    if (Test-Path "{{PUBLISH_DIR}}/source") { Remove-Item -Path "{{PUBLISH_DIR}}/source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/source" -Force | Out-Null

#
# Publish tasks:
#
publish-dry package="divoom":
    cargo publish --dry-run -p {{package}}

    @echo "Files in package:"
    cargo package --list -p {{package}}

publish package="divoom":
    cargo publish -p {{package}}