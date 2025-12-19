# Gridix COPR Package

This repository contains the RPM spec file for building Gridix on Fedora/RHEL via COPR.

## Installation (Fedora)

```bash
sudo dnf copr enable mcb-smart-boy/gridix
sudo dnf install gridix
```

## Building locally

```bash
rpmbuild -ba gridix.spec
```

## About Gridix

A fast, secure, cross-platform database management tool with Helix/Vim keybindings.

- GitHub: https://github.com/MCB-SMART-BOY/Gridix
