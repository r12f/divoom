$ErrorActionPreference = 'Stop'

$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = $toolsDir
Write-Host "{binary.name} is going to be installed in '$installDir'"

$packageArgs = @{
  PackageName    = $env:ChocolateyPackageName
  UnzipLocation  = $installDir
  Url64bit       = 'https://github.com/r12f/divoom/releases/download/{build.version}/{binary.name}.{build.version}.windows.x64.zip'
  Checksum64     = '{packages.{package.name.raw}.binary.windows.x64}'
  ChecksumType64 = 'sha256'
  Url            = 'https://github.com/r12f/divoom/releases/download/{build.version}/{binary.name}.{build.version}.windows.x86.zip'
  Checksum       = '{packages.{package.name.raw}.binary.windows.x86}'
  ChecksumType   = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs