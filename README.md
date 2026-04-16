# presetable
A tool for command alias presets

## Installation
### cargo
```bash
cargo install presetable
```
### Manual build
```bash
cargo build --release
# The file will be in ./target/release/presetable
```
### NixOS
Add presetable to your nix flake inputs:
```nix
inputs.presetable.url = "github:quixaq/presetable";
```
Include the package in your config:
```nix
environment.systemPackages = [ inputs.presetable.packages.${pkgs.system}.default ];
```

## Configuration
### config.toml
An example configuration can be seen in `config.toml.example`. Put it under `~/.config/presetable/config.toml`
### NixOS(via Home Manager)
```nix
xdg.configFile."presetable/config.toml".text = ''
  # include the config.toml here
'';
```
