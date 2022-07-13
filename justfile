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
  } else if BUILD_TARGET == "linux-x86" {
    "i686-unknown-linux-gnu"
  } else if BUILD_TARGET == "linux-x64" {
    "x86_64-unknown-linux-gnu"
  } else if BUILD_TARGET == "linux-arm" {
    "arm-unknown-linux-gnueabi"
  } else if BUILD_TARGET == "linux-aarch64" {
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

# Signing settings
export BUILD_SIGNING_URL := env_var_or_default("BUILD_SIGNING_URL", "")
export BUILD_SIGNING_VAULT_URL := env_var_or_default("BUILD_SIGNING_VAULT_URL", "")
export BUILD_SIGNING_TENANT_ID := env_var_or_default("BUILD_SIGNING_TENANT_ID", "")
export BUILD_SIGNING_CLIENT_ID := env_var_or_default("BUILD_SIGNING_CLIENT_ID", "")
export BUILD_SIGNING_CLIENT_SECRET := env_var_or_default("BUILD_SIGNING_CLIENT_SECRET", "")
export BUILD_SIGNING_CERT_NAME := env_var_or_default("BUILD_SIGNING_CERT_NAME", "")

# Publish
PUBLISH_DIR := justfile_directory() + "/publish"
PUBLISH_PACKAGES_DIR := PUBLISH_DIR + "/packages"
PUBLISH_CHECKSUMS_DIR := PUBLISH_DIR + "/checksums"

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
fix-ver:
    @("divoom", "divoom_cli") | ForEach-Object { \
      $cargoFilePath = "{{(justfile_directory())}}/$_/Cargo.toml"; \
      Write-Host "Updating version in file: $cargoFilePath"; \
      (Get-Content $cargoFilePath) -Replace '"0.0.1"', '"{{BUILD_VERSION}}"' | Set-Content $cargoFilePath; \
    }

build:
    cargo build --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}

doc:
    cargo doc

test:
    cargo test --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}


#
# Install task:
#
install:
    cargo install --profile release --path ./divoom_cli

#
# Pack tasks:
#
pack-prepare PACKAGE="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}" -Force | Out-Null
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" -Force | Out-Null

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt") { Remove-Item -ItemType File -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt" }
    echo "build.os={{BUILD_OS}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "build.arch={{BUILD_ARCH}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "build.target={{BUILD_TOOL_TARGET}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "build.version={{BUILD_VERSION}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "build.out_dir={{BUILD_OUTPUT_FOLDER}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "binary.name={{replace(PACKAGE, '_', '-')}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.name.pascal_case=$([regex]::Replace("{{PACKAGE}}", '(?i)(?:^|_)(\p{L})', { $args[0].Groups[1].Value.ToUpper() }))" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.description=$(gc {{justfile_directory()}}/{{PACKAGE}}/Cargo.toml | sls 'description = "(..+)"' | % { $_.Matches[0].Groups[1].Value })" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.tags=divoom;pixoo;pixoo64" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"

pack-source:
    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish/source") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/build" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/divoom" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/divoom_cli" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/Cargo.*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/justfile" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/source" -Recurse

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    if (Test-Path "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip") { Remove-Item -Path "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip" -Recurse -Force }
    7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip" "./{{BUILD_OUTPUT_FOLDER}}/publish/source/*"

    just gen-checksum "packages.source" "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip";

pack-binary PACKAGE="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin" -Force | Out-Null

    $fileNames = @("{{replace(PACKAGE, '_', '-')}}.exe", "{{replace(PACKAGE, '_', '-')}}"); \
    foreach ($fileName in $fileNames) { \
      $filePath = "{{BUILD_OUTPUT_FOLDER}}/$fileName"; \
      if (Test-Path $filePath) { \
        if ("{{BUILD_OS}}" -eq "windows") { just sign-file "$filePath"; } \
        \
        Write-Host "Copying binary file: $filePath"; \
        Copy-Item -Path $filePath -Destination "{{PUBLISH_DIR}}/{{PACKAGE}}/bin"; \
        just gen-checksum "binary.{{PACKAGE}}.{{BUILD_OS}}.{{BUILD_ARCH}}" $filePath; \
      } \
    }

pack-symbols PACKAGE="divoom_cli":
    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols" -Force | Out-Null

    $fileNames = @("{{PACKAGE}}.pdb", "{{PACKAGE}}.debug" ); \
    foreach ($fileName in $fileNames) { \
      $filePath = "{{BUILD_OUTPUT_FOLDER}}/$fileName"; \
      if (Test-Path $filePath) { \
        Write-Host "Copying symbol file: $filePath"; \
        Copy-Item -Path $filePath -Destination "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols"; \
        just gen-checksum "symbols.{{PACKAGE}}.{{BUILD_OS}}.{{BUILD_ARCH}}" $filePath; \
      } \
    }

pack-binary-zip PACKAGE="divoom_cli":
    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }

    if ("{{BUILD_OS}}" -eq "windows") { \
        $packageName = "{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.zip"; \
        7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*"; \
        just gen-checksum "packages.{{PACKAGE}}.binary.{{BUILD_OS}}.{{BUILD_ARCH}}" "{{PUBLISH_PACKAGES_DIR}}/${packageName}"; \
    }

    if ("{{BUILD_OS}}" -ne "windows") { \
        $packageName = "{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.tar"; \
        7z -ttar a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*"; \
        7z -tgzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName.gz" "{{PUBLISH_PACKAGES_DIR}}/$packageName"; \
        Remove-Item "{{PUBLISH_PACKAGES_DIR}}/$packageName"; \
        $packageName = "${packageName}.gz"; \
        just gen-checksum "packages.{{PACKAGE}}.binary.{{BUILD_OS}}.{{BUILD_ARCH}}" "{{PUBLISH_PACKAGES_DIR}}/${packageName}"; \
    }

pack-msix PACKAGE="divoom_cli":
    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish/msix") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish/msix" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish/msix/bin" -Force | Out-Null
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish/msix/assets" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/assets/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/msix/assets"
    Copy-Item -Path "{{justfile_directory()}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/msix/bin"
    Copy-Item -Path "{{justfile_directory()}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/msix/bin"
    Copy-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/msix/bin"

    just eval-template "{{justfile_directory()}}/build/package-templates/msix/appxmanifest.xml" \
      "{{BUILD_OUTPUT_FOLDER}}/publish/msix/appxmanifest.xml" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    just eval-template "{{justfile_directory()}}/build/package-templates/msix/appxmappings.txt" \
      "{{BUILD_OUTPUT_FOLDER}}/publish/msix/appxmappings.txt" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    & "C:/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64/makeappx.exe" pack /m "{{BUILD_OUTPUT_FOLDER}}/publish/msix/appxmanifest.xml" \
      /f "{{BUILD_OUTPUT_FOLDER}}/publish/msix/appxmappings.txt" \
      /p "{{BUILD_OUTPUT_FOLDER}}/publish/msix/{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.msix"

    just sign-file "{{BUILD_OUTPUT_FOLDER}}/publish/msix/{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.msix"

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    Copy-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish/msix/*.msix" -Destination "{{PUBLISH_PACKAGES_DIR}}" -Force

pack-nuget PACKAGE="divoom_cli":
    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish/nuget") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish/nuget" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish/nuget" -Force | Out-Null
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/content" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/content"
    Copy-Item -Path "{{justfile_directory()}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/content"
    Copy-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/content"

    just eval-template "{{justfile_directory()}}/build/package-templates/nuget/nupkg.csproj" \
      "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/nupkg.csproj" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    dotnet pack "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/nupkg.csproj" -o "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/output"

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    Copy-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish/nuget/output/*.nupkg" -Destination "{{PUBLISH_PACKAGES_DIR}}" -Force


#
# Publish tasks:
#
publish-dry PACKAGE="divoom":
    cargo publish --dry-run -p {{PACKAGE}}

    @echo "Files in package:"
    cargo package --list -p {{PACKAGE}}

publish PACKAGE="divoom":
    cargo publish -p {{PACKAGE}}


#
# Utility tasks:
#
sign-file FILE_PATH:
    Write-Host "Signing file: {{FILE_PATH}}"

    if (-not [string]::IsNullOrEmpty($BUILD_SIGNING_URL)) { \
        AzureSignTool sign \
            -du "$BUILD_SIGNING_URL" \
            -kvu "$BUILD_SIGNING_VAULT_URL" \
            -kvt "$BUILD_SIGNING_TENANT_ID" \
            -kvi "$BUILD_SIGNING_CLIENT_ID" \
            -kvs "$BUILD_SIGNING_CLIENT_SECRET" \
            -kvc "$BUILD_SIGNING_CERT_NAME" \
            -v "{{FILE_PATH}}"; \
    } else { \
        Write-Host "Skipped signing file, because signing settings are not set."; \
    }

gen-checksum INPUT_FILE_ID INPUT_FILE_PATH:
    if (-not (Test-Path "{{PUBLISH_CHECKSUMS_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_CHECKSUMS_DIR}}" -Force | Out-Null }

    $fileName = [System.IO.Path]::GetFileName("{{INPUT_FILE_PATH}}"); \
    $checksumFilePath = "{{PUBLISH_CHECKSUMS_DIR}}/{{INPUT_FILE_ID}}.checksum.txt"; \
    Write-Host "Generating checksum file: $checksumFilePath"; \
    echo "{{INPUT_FILE_ID}}=$((Get-FileHash "{{INPUT_FILE_PATH}}" -Algorithm SHA256).Hash.ToLowerInvariant())" > $checksumFilePath;

eval-template TEMPLATE OUTPUT_FILE +TEMPLATE_PARAMETER_FOLDERS:
    Write-Host "Reading all template parameters ..."; \
    $allParameters = foreach ($templateParameterFolder in "{{TEMPLATE_PARAMETER_FOLDERS}}".Split(" ")) { \
        Get-ChildItem "$templateParameterFolder/*.txt" | % { Get-Content $_ }; \
    }; \
    Write-Host ""; \
    Write-Host "All template parameters:"; \
    $allParameters; \
    Write-Host ""; \
    \
    Write-Host "Generating content ..."; \
    $templateFileContent = Get-Content "{{TEMPLATE}}" -Raw; \
    $allParameters | ForEach-Object { \
        $parameterPair = $_.Split("="); \
        $templateFileContent = $templateFileContent.Replace("{" + $parameterPair[0] + "}", $parameterPair[1]); \
    }; \
    Write-Host "Writing content to file: {{OUTPUT_FILE}}"; \
    $utf8NoBom = New-Object System.Text.UTF8Encoding $False; \
    [System.IO.File]::WriteAllLines("{{OUTPUT_FILE}}", $templateFileContent, $utf8NoBom);
