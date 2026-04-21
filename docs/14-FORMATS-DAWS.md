# Formats & DAW Compatibility

Smoothie Elite guarantees compile-once, deploy-everywhere architecture.

## 1. VST3 (`smoothie-vst3`)

The standard for modern plugin development.
- Implements the Steinberg VST3 ABI.
- Supports full parameter automation, sample-accurate changes, and MIDI CC.
- Tested and verified in: **Ableton Live 11+, FL Studio 21+, Cubase 12+, Studio One 6+, Reaper.**

## 2. CLAP (`smoothie-clap`)

CLEVER AUDIO PLUG-IN. The modern open standard.
- Supports **Polyphonic Expression** and **Non-destructive Automation**.
- Exceptionally fast parameter lookup and thread-pool sharing.
- Tested and verified in: **Bitwig Studio, Reaper.**

## 3. Audio Units (`smoothie-au`)

Apple's native format for macOS and iOS.
- Implements AUv2 (and experimental AUv3).
- Tested and verified in: **Logic Pro X, GarageBand.**

## 4. AAX (`smoothie-aax`)

Avid's format for Pro Tools.
- Note: Requires the proprietary Avid AAX SDK. If the SDK is present in the environment variables during build, the `xtask` system will automatically bundle the AAX format.

## 5. Standalone

Builds the plugin as a standard desktop application.
- Uses **Tauri v2** for the application shell.
- Interfaces directly with system audio drivers (CoreAudio, WASAPI, ALSA) via CPAL.
- Useful for live performance or users without a DAW.
