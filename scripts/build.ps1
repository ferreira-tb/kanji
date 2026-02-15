param(
  [switch]$Android
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run type-check

if ($Android) {
  cargo tauri android build --apk

  $PackageJson = Get-Content -Path 'package.json' -Raw | ConvertFrom-Json -AsHashtable
  $Version = $PackageJson['version']

  $Path = 'src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk'
  kanata add -p $Path -n "kanji-$Version.apk"
}
else {
  cargo tauri build
}
