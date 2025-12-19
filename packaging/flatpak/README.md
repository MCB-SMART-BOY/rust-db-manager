# Gridix Flatpak

This repository contains the Flatpak manifest for Gridix.

## Building locally

```bash
flatpak-builder --user --install --force-clean build-dir io.github.mcb_smart_boy.Gridix.yaml
```

## Running

```bash
flatpak run io.github.mcb_smart_boy.Gridix
```

## Submitting to Flathub

1. Fork https://github.com/flathub/flathub
2. Create a new repository named `io.github.mcb_smart_boy.Gridix`
3. Add this manifest and related files
4. Submit a PR to flathub/flathub

## About Gridix

A fast, secure, cross-platform database management tool with Helix/Vim keybindings.

- GitHub: https://github.com/MCB-SMART-BOY/Gridix
