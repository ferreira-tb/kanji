param(
  [switch]$Android
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Android) {
  cargo tauri android build --apk true
}
else {
  cargo tauri build
}
