#!/usr/bin/env just --justfile

# Use powershell 7 on all platforms.
set shell := ["pwsh", "-NoLogo", "-Command"]

# Load .env if we have it. .env files are ignored in check-ins, so we can have private dev environment.
set dotenv-load

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
} else if env_var_or_default("BUILD_ARCH", arch()) == "aarch64" {
    "arm64"
} else {
    env_var_or_default("BUILD_ARCH", arch())
}

SNAP_PACKAGE_ARCH := if BUILD_ARCH == "arm" {
    "armhf"
} else if BUILD_ARCH == "x64" {
    "amd64"
} else {
    BUILD_ARCH
}

OS_ARCH := if arch() == "x86_64" {
    "x64"
} else if arch() == "aarch64" {
    "arm64"
} else {
    arch()
}

BUILD_TARGET := BUILD_OS + "-" + BUILD_ARCH

BUILD_TOOL_TARGET := if BUILD_TARGET == "windows-x86" {
    "i686-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-x64" {
    "x86_64-pc-windows-msvc"
  } else if BUILD_TARGET == "windows-arm64" {
    "aarch64-pc-windows-msvc"
  } else if BUILD_TARGET == "linux-x86" {
    "i686-unknown-linux-gnu"
  } else if BUILD_TARGET == "linux-x64" {
    "x86_64-unknown-linux-gnu"
  } else if BUILD_TARGET == "linux-arm" {
    "arm-unknown-linux-gnueabi"
  } else if BUILD_TARGET == "linux-arm64" {
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
BUILD_IS_CROSS_COMPILE := if BUILD_ARCH != OS_ARCH { "true" } else { "false" }

# Signing settings
export BUILD_SIGNING_URL := env_var_or_default("BUILD_SIGNING_URL", "")
export BUILD_SIGNING_VAULT_URL := env_var_or_default("BUILD_SIGNING_VAULT_URL", "")
export BUILD_SIGNING_TENANT_ID := env_var_or_default("BUILD_SIGNING_TENANT_ID", "")
export BUILD_SIGNING_CLIENT_ID := env_var_or_default("BUILD_SIGNING_CLIENT_ID", "")
export BUILD_SIGNING_CLIENT_SECRET := env_var_or_default("BUILD_SIGNING_CLIENT_SECRET", "")
export BUILD_SIGNING_CERT_NAME := env_var_or_default("BUILD_SIGNING_CERT_NAME", "")

# Publish
PUBLISH_DIR := "./publish"
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
    sudo apt install -y build-essential libc6-dev libssl-dev p7zip-full

    case "{{BUILD_TOOL_TARGET}}" in
        "i686-unknown-linux-gnu")
            # For building x86 binary, we are using gcc-multilib.
            # This package is conflicting with other gcc-* packages, but we don not know any better package to use.
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

    @echo "Installing cross compile environment:"
    cargo install cross --git https://github.com/cross-rs/cross

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
    cargo clippy --all-targets --all-features

lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty


#
# Build / test tasks:
#
fix-ver:
    @("divoom", "divoom_cli", "divoom_gateway") | ForEach-Object { \
      $cargoFilePath = "{{(justfile_directory())}}/$_/Cargo.toml"; \
      Write-Host "Updating version in file: $cargoFilePath"; \
      (Get-Content $cargoFilePath) -Replace '"0.0.1"', '"{{BUILD_VERSION}}"' | Set-Content $cargoFilePath; \
    }

build:
    if ("{{BUILD_OS}}" -eq "linux" -and "{{BUILD_IS_CROSS_COMPILE}}" -eq "true") { \
        Write-Host "Build with cross for cross build: {{BUILD_TOOL_TARGET}}"; \
        cross build --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}; \
    } else { \
        Write-Host "Build with cargo for regular build: {{BUILD_TOOL_TARGET}}"; \
        cargo build --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}; \
    }

doc:
    cargo doc

test:
    if ("{{BUILD_OS}}" -eq "linux" -and "{{BUILD_IS_CROSS_COMPILE}}" -eq "true") { \
        Write-Host "Test with cross for cross tests: {{BUILD_TOOL_TARGET}}"; \
        cross test --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}; \
    } else { \
        Write-Host "Test with cargo for regular tests: {{BUILD_TOOL_TARGET}}"; \
        cargo test --profile {{BUILD_PROFILE}} --target {{BUILD_TOOL_TARGET}}; \
    }

make-symbols:
    if ("{{BUILD_OS}}" -eq "windows") { \
        Write-Host "No need to manually make symbol files on windows. Pdb files are generated by default."; \
        return; \
    } elseif ("{{BUILD_OS}}" -eq "linux") { \
        $gccXCompilerPrefix = ""; \
        if ("{{BUILD_IS_CROSS_COMPILE}}" -eq "true") { \
            $gccXCompilerPrefix = "{{BUILD_TOOL_TARGET}}-"; \
            if ("{{BUILD_TOOL_TARGET}}" -eq "i686-unknown-linux-gnu") { \
                $gccXCompilerPrefix = "x86_64-linux-gnu-"; \
            } elseif ("{{BUILD_TOOL_TARGET}}" -eq "arm-unknown-linux-gnueabi") { \
                $gccXCompilerPrefix = "arm-linux-gnueabi-"; \
            } elseif ("{{BUILD_TOOL_TARGET}}" -eq "aarch64-unknown-linux-gnu") { \
                $gccXCompilerPrefix = "aarch64-linux-gnu-"; \
            } \
        } \
        $objcopyPath = "${gccXCompilerPrefix}objcopy"; \
        $stripPath = "${gccXCompilerPrefix}strip"; \
    } elseif ("{{BUILD_OS}}" -eq "macos") { \
        $objcopyPath = "/usr/local/opt/binutils/bin/gobjcopy"; \
        $stripPath = "strip"; \
    } \
    Write-Host "Striping binary with objcopy = $objcopyPath, strip = $stripPath."; \
    \
    @("divoom-cli") | ForEach-Object { just _make-symbols $objcopyPath $stripPath $_; }

_make-symbols OBJCOPY_PATH="objcopy" STRIP_PATH="strip" FILE_NAME="divoom-cli":
    Write-Host "Making symbol file for {{FILE_NAME}}";

    Write-Host "Removing existing symbol file: {{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}.debug";
    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}.debug") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}.debug"; }

    Write-Host "Generating new symbol file: {{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}";
    & {{OBJCOPY_PATH}} --only-keep-debug "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}" "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}.debug";

    if ("{{BUILD_OS}}" -eq "macos") { & {{STRIP_PATH}} -S "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}"; } \
    else { & {{STRIP_PATH}} --strip-debug --strip-unneeded -p "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}"; }

    & {{OBJCOPY_PATH}} --add-gnu-debuglink="{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}.debug" "{{BUILD_OUTPUT_FOLDER}}/{{FILE_NAME}}";

#
# Install task:
#
install:
    cargo install --profile release --path ./divoom_cli
    cargo install --profile release --path ./divoom_gateway

#
# Pack tasks:
#
pack-prepare-all:
    just pack-prepare divoom_cli
    just pack-prepare divoom_gateway

pack-prepare PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

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
    echo "package.name.raw={{PACKAGE}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.name.pascal_case=$([regex]::Replace("{{PACKAGE}}", '(?i)(?:^|_)(\p{L})', { $args[0].Groups[1].Value.ToUpper() }))" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.description=$(gc {{justfile_directory()}}/{{PACKAGE}}/Cargo.toml | sls 'description = "(..+)"' | % { $_.Matches[0].Groups[1].Value })" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.tags=divoom;pixoo;pixoo64" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"
    echo "package.arch.snap={{SNAP_PACKAGE_ARCH}}" >> "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters/parameters.txt"

pack-source:
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/assets" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/build" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/divoom" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/divoom_cli" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/divoom_gateway" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/Cargo.*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/justfile" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse
    Copy-Item -Path "{{justfile_directory()}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source" -Recurse

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    if (Test-Path "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip") { Remove-Item -Path "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip" -Recurse -Force }
    7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip" "./{{BUILD_OUTPUT_FOLDER}}/publish-prepare/source/*"

    just gen-checksum "packages.source" "{{PUBLISH_PACKAGES_DIR}}/divoom.source.{{BUILD_VERSION}}.zip";

pack-post-build:
    Copy-Item -Path "{{justfile_directory()}}/build/post-build/post-build.justfile" -Destination "{{PUBLISH_DIR}}"

pack-binary-all:
    just pack-binary divoom_cli
    just pack-binary divoom_gateway

pack-binary PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

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

pack-symbols-all:
    just pack-symbols divoom_cli
    just pack-symbols divoom_gateway

pack-symbols PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols" -Force | Out-Null

    $fileNames = @("{{PACKAGE}}.pdb", "{{replace(PACKAGE, '_', '-')}}.debug"); \
    foreach ($fileName in $fileNames) { \
      $filePath = "{{BUILD_OUTPUT_FOLDER}}/$fileName"; \
      if (Test-Path $filePath) { \
        Write-Host "Copying symbol file: $filePath"; \
        Copy-Item -Path $filePath -Destination "{{PUBLISH_DIR}}/{{PACKAGE}}/symbols"; \
        just gen-checksum "symbols.{{PACKAGE}}.{{BUILD_OS}}.{{BUILD_ARCH}}" $filePath; \
      } \
    }

pack-binary-zip-all:
    just pack-binary-zip divoom_cli
    just pack-binary-zip divoom_gateway

pack-binary-zip PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }

    if ("{{BUILD_OS}}" -eq "windows") { \
        $packageName = "{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.zip"; \
        7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*"; \
        7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{justfile_directory()}}/{{PACKAGE}}/LICENSE"; \
        7z -tzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{justfile_directory()}}/{{PACKAGE}}/README.md"; \
        just gen-checksum "packages.{{PACKAGE}}.binary.{{BUILD_OS}}.{{BUILD_ARCH}}" "{{PUBLISH_PACKAGES_DIR}}/${packageName}"; \
    }

    if ("{{BUILD_OS}}" -ne "windows") { \
        $packageName = "{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.tar"; \
        7z -ttar a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*"; \
        7z -ttar a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{justfile_directory()}}/{{PACKAGE}}/LICENSE"; \
        7z -ttar a "{{PUBLISH_PACKAGES_DIR}}/$packageName" "{{justfile_directory()}}/{{PACKAGE}}/README.md"; \
        7z -tgzip a "{{PUBLISH_PACKAGES_DIR}}/$packageName.gz" "{{PUBLISH_PACKAGES_DIR}}/$packageName"; \
        Remove-Item "{{PUBLISH_PACKAGES_DIR}}/$packageName"; \
        $packageName = "${packageName}.gz"; \
        just gen-checksum "packages.{{PACKAGE}}.binary.{{BUILD_OS}}.{{BUILD_ARCH}}" "{{PUBLISH_PACKAGES_DIR}}/${packageName}"; \
    }

pack-msix-all:
    just pack-msix divoom_cli
    just pack-msix divoom_gateway

pack-msix PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/bin" -Force | Out-Null
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/assets" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/assets/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/assets"
    Copy-Item -Path "{{justfile_directory()}}/{{PACKAGE}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/bin"
    Copy-Item -Path "{{justfile_directory()}}/{{PACKAGE}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/bin"
    Copy-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/bin"

    just eval-template "{{justfile_directory()}}/build/package-templates/msix/appxmanifest.xml" \
      "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/appxmanifest.xml" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    just eval-template "{{justfile_directory()}}/build/package-templates/msix/appxmappings.txt" \
      "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/appxmappings.txt" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    & "C:/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64/makeappx.exe" pack /m "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/appxmanifest.xml" \
      /f "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/appxmappings.txt" \
      /p "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.msix"

    just sign-file "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/{{replace(PACKAGE, '_', '-')}}.{{BUILD_VERSION}}.{{BUILD_OS}}.{{BUILD_ARCH}}.msix"

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    Copy-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/msix/*.msix" -Destination "{{PUBLISH_PACKAGES_DIR}}" -Force

pack-nuget-all:
    just pack-nuget divoom_cli
    just pack-nuget divoom_gateway

pack-nuget PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget") { Remove-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget" -Force | Out-Null
    New-Item -ItemType Directory -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/content" -Force | Out-Null

    Copy-Item -Path "{{justfile_directory()}}/{{PACKAGE}}/LICENSE" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/content"
    Copy-Item -Path "{{justfile_directory()}}/{{PACKAGE}}/README.md" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/content"
    Copy-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/bin/*" -Destination "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/content"

    just eval-template "{{justfile_directory()}}/build/package-templates/nuget/nupkg.csproj" \
      "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/nupkg.csproj" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

    dotnet pack "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/nupkg.csproj" -o "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/output"

    if (-not (Test-Path "{{PUBLISH_PACKAGES_DIR}}")) { New-Item -ItemType Directory -Path "{{PUBLISH_PACKAGES_DIR}}" -Force | Out-Null }
    Copy-Item -Path "{{BUILD_OUTPUT_FOLDER}}/publish-prepare/nuget/output/*.nupkg" -Destination "{{PUBLISH_PACKAGES_DIR}}" -Force

pack-choco-all:
    just pack-choco divoom_cli
    just pack-choco divoom_gateway

pack-choco PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/choco-source") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/choco-source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/choco-source" -Force | Out-Null

    just eval-template-dir "{{justfile_directory()}}/build/package-templates/chocolatey" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/choco-source" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

pack-scoop-all:
    just pack-scoop divoom_cli
    just pack-scoop divoom_gateway

pack-scoop PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/scoop-source") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/scoop-source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/scoop-source" -Force | Out-Null

    just eval-template "{{justfile_directory()}}/build/package-templates/scoop/scoop.json" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/scoop-source/{{replace(PACKAGE, '_', '-')}}.json" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

pack-snap-all:
    just pack-snap divoom_cli
    just pack-snap divoom_gateway

pack-snap PACKAGE="divoom_cli":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/snap-source") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/snap-source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/snap-source/{{replace(PACKAGE, '_', '-')}}.{{BUILD_ARCH}}/snap" -Force | Out-Null

    just eval-template "{{justfile_directory()}}/build/package-templates/snap/snapcraft.yaml" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/snap-source/{{replace(PACKAGE, '_', '-')}}.{{BUILD_ARCH}}/snap/snapcraft.yaml" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

pack-docker-all:
    just pack-docker divoom_gateway

pack-docker PACKAGE="divoom_gateway":
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    if (Test-Path "{{PUBLISH_DIR}}/{{PACKAGE}}/docker-source") { Remove-Item -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/docker-source" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{PUBLISH_DIR}}/{{PACKAGE}}/docker-source/{{replace(PACKAGE, '_', '-')}}.{{BUILD_ARCH}}" -Force | Out-Null

    just eval-template "{{justfile_directory()}}/build/package-templates/docker/Dockerfile" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/docker-source/{{replace(PACKAGE, '_', '-')}}.{{BUILD_ARCH}}/Dockerfile" \
      "{{PUBLISH_DIR}}/{{PACKAGE}}/template-parameters" \
      "{{PUBLISH_CHECKSUMS_DIR}}"

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

    if (-not [string]::IsNullOrEmpty(${env:BUILD_SIGNING_URL})) { \
        AzureSignTool sign \
            -du "${env:BUILD_SIGNING_URL}" \
            -kvu "${env:BUILD_SIGNING_VAULT_URL}" \
            -kvt "${env:BUILD_SIGNING_TENANT_ID}" \
            -kvi "${env:BUILD_SIGNING_CLIENT_ID}" \
            -kvs "${env:BUILD_SIGNING_CLIENT_SECRET}" \
            -kvc "${env:BUILD_SIGNING_CERT_NAME}" \
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

eval-template-dir TEMPLATE_FOLDER OUTPUT_FOLDER +TEMPLATE_PARAMETER_FOLDERS:
    @Write-Host "Current invocation directory: {{invocation_directory()}}"

    Get-ChildItem "{{TEMPLATE_FOLDER}}/*" -Recurse -File | ForEach-Object { \
        $relativePath = [System.IO.Path]::GetRelativePath("{{TEMPLATE_FOLDER}}", $_); \
        $relativeFolder = [System.IO.Path]::GetDirectoryName($relativePath); \
        if (-not (Test-Path "{{OUTPUT_FOLDER}}/$relativeFolder")) { New-Item -ItemType Directory -Path "{{OUTPUT_FOLDER}}/$relativeFolder" -Force | Out-Null } \
        \
        just eval-template "$($_.FullName)" \
          "{{OUTPUT_FOLDER}}/$relativePath" \
          {{TEMPLATE_PARAMETER_FOLDERS}}; \
    }

eval-template TEMPLATE OUTPUT_FILE +TEMPLATE_PARAMETER_FOLDERS:
    @Write-Host "Current invocation directory: {{invocation_directory()}}"
    @Write-Host "Reading all template parameters ..."; \
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
    while ($true) { \
        $newTemplateFileContent = $templateFileContent; \
        $allParameters | ForEach-Object { \
            $parameterPair = $_.Split("="); \
            $newTemplateFileContent = $newTemplateFileContent.Replace("{" + $parameterPair[0] + "}", $parameterPair[1]); \
        }; \
        if ($templateFileContent -eq $newTemplateFileContent) { \
            break; \
        } else { \
            $templateFileContent = $newTemplateFileContent; \
        } \
    } \
    Write-Host "Content generated:"; \
    Write-Host "$templateFileContent"; \
    Write-Host ""; \
    Write-Host "Writing content to file: {{OUTPUT_FILE}}"; \
    $utf8NoBom = New-Object System.Text.UTF8Encoding $False; \
    [System.IO.File]::WriteAllLines("{{OUTPUT_FILE}}", $templateFileContent, $utf8NoBom);
