#!/usr/bin/env just --justfile

# Use powershell 7 on all platforms.
set shell := ["pwsh", "-NoLogo", "-Command"]

# Load .env if we have it. .env files are ignored in check-ins, so we can have private dev environment.
set dotenv-load

# Global variables
BUILD_VERSION := env_var_or_default("BUILD_VERSION", "")
BUILD_FOLDER_PREFIX := "./Build.Build."
BUILD_MODULES := "divoom_cli"

INTERMEDIATE_FOLDER := "./temp"
INTERMEDIATE_SYMBOL_PACKAGE_FOLDER := INTERMEDIATE_FOLDER + "/symbols"
INTERMEDIATE_CHOCO_PACKAGE_FOLDER := INTERMEDIATE_FOLDER + "/choco"

RELEASE_FOLDER := "./release"
RELEASE_TEMPLATE_PARAMETER_FOLDER := RELEASE_FOLDER + "/template-parameters"
RELEASE_CRATES_FOLDER := RELEASE_FOLDER + "/crates"
RELEASE_NUGET_FOLDER := RELEASE_FOLDER + "/nuget"
RELEASE_GITHUB_FOLDER := RELEASE_FOLDER + "/github"
RELEASE_CHOCO_FOLDER := RELEASE_FOLDER + "/choco"

#
# Preparation tasks
#
prepare-release-prepare:
    if (Test-Path "{{RELEASE_TEMPLATE_PARAMETER_FOLDER}}") { Remove-Item -Path "{{RELEASE_TEMPLATE_PARAMETER_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{RELEASE_TEMPLATE_PARAMETER_FOLDER}}" -Force | Out-Null

    @Write-Host "Copy all template parameters from each build folder ..."
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/checksums/*" -Recurse | Copy-Item -Destination "{{RELEASE_TEMPLATE_PARAMETER_FOLDER}}" -PassThru

#
# Pack symbols
#
pack-symbols:
    if (Test-Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}") { Remove-Item -Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}" -Force | Out-Null

    just _pack-symbols-with-arch windows x86
    just _pack-symbols-with-arch windows x64
    just _pack-symbols-with-arch windows arm64

    just _pack-symbols-with-arch linux x86
    just _pack-symbols-with-arch linux x64
    just _pack-symbols-with-arch linux arm
    just _pack-symbols-with-arch linux arm64

    just _pack-symbols-with-arch macos x64

    7z -tzip a "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/divoom.symbols.{{BUILD_VERSION}}.zip" "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/*"

_pack-symbols-with-arch BUILD_OS BUILD_ARCH:
    @Write-Host "Copying all symbols to intermediate folder: OS = {{BUILD_OS}}, Arch = {{BUILD_ARCH}}"

    if (Test-Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/{{BUILD_OS}}.{{BUILD_ARCH}}") { Remove-Item -Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/{{BUILD_OS}}.{{BUILD_ARCH}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/{{BUILD_OS}}.{{BUILD_ARCH}}" -Force | Out-Null

    Copy-Item -Path "{{BUILD_FOLDER_PREFIX}}{{BUILD_OS}}{{BUILD_ARCH}}/*/symbols/*" -Destination "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/{{BUILD_OS}}.{{BUILD_ARCH}}" -Force -PassThru

#
# Pack choco
#
pack-choco:
    if (Test-Path "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}") { Remove-Item -Path "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}" -Force | Out-Null

    if (Test-Path "{{RELEASE_CHOCO_FOLDER}}") { Remove-Item -Path "{{RELEASE_CHOCO_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{RELEASE_CHOCO_FOLDER}}" -Force | Out-Null

    just _pack-choco-with-package divoom_cli

_pack-choco-with-package PACKAGE="divoom_cli":
    if (-not (Test-Path "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}/{{PACKAGE}}")) { \
        New-Item -ItemType Directory -Path "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}/{{PACKAGE}}" -Force | Out-Null; \
    }

    just eval-template-dir "{{BUILD_FOLDER_PREFIX}}windowsx64/{{PACKAGE}}/choco-source" \
        "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}/{{PACKAGE}}" \
        "{{RELEASE_TEMPLATE_PARAMETER_FOLDER}}" \

    @Write-Host "Generating final chocolatey package to {{RELEASE_CHOCO_FOLDER}} ..."
    choco pack "{{INTERMEDIATE_CHOCO_PACKAGE_FOLDER}}/{{PACKAGE}}/choco.nuspec" --outputdirectory "{{RELEASE_CHOCO_FOLDER}}"

#
# Prepare packages for crate.io release
#
prepare-crate-io-release:
    if (Test-Path "{{RELEASE_CRATES_FOLDER}}") { Remove-Item -Path "{{RELEASE_CRATES_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{RELEASE_CRATES_FOLDER}}" -Force | Out-Null

    @Write-Host "Copy all source tarballs from each build folder ..."
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/packages/*.source.*.zip" -Recurse | Copy-Item -Destination "{{RELEASE_CRATES_FOLDER}}" -PassThru

#
# Prepare packages for github releases
#
prepare-github-release:
    if (Test-Path "{{RELEASE_GITHUB_FOLDER}}") { Remove-Item -Path "{{RELEASE_GITHUB_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{RELEASE_GITHUB_FOLDER}}" -Force | Out-Null

    @Write-Host "Copy all tarballs from each build folder ..."
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/packages/*.zip" -Recurse | Copy-Item -Destination "{{RELEASE_GITHUB_FOLDER}}" -PassThru
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/packages/*.tar.gz" -Recurse | Copy-Item -Destination "{{RELEASE_GITHUB_FOLDER}}" -PassThru
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/packages/*.msix" -Recurse | Copy-Item -Destination "{{RELEASE_GITHUB_FOLDER}}" -PassThru

    @Write-Host "Copy symbol package ..."
    Copy-Item -Path "{{INTERMEDIATE_SYMBOL_PACKAGE_FOLDER}}/divoom.symbols.{{BUILD_VERSION}}.zip" -Destination "{{RELEASE_GITHUB_FOLDER}}" -PassThru

#
# Prepare packages for nuget release
#
prepare-nuget-release:
    if (Test-Path "{{RELEASE_NUGET_FOLDER}}") { Remove-Item -Path "{{RELEASE_NUGET_FOLDER}}" -Recurse -Force }
    New-Item -ItemType Directory -Path "{{RELEASE_NUGET_FOLDER}}" -Force | Out-Null

    @Write-Host "Copy all nuget packages from each build folder ..."
    Get-ChildItem "./{{BUILD_FOLDER_PREFIX}}*/packages/*.nupkg" -Recurse | Copy-Item -Destination "{{RELEASE_NUGET_FOLDER}}" -PassThru

#
# Utility tasks
#
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
