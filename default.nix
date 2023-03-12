let
  # base-nixpkgs = import <nixpkgs> {};
  mozillaOverlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust = (nixpkgs.rustChannelOf { channel = "nightly"; }).rust.override {
    targets = [ "x86_64-unknown-linux-gnu" ];
  };
  rustPlatform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };

in rustPlatform.buildRustPackage {
  name = "new-termainl-hyprland";
  src = ./.;
  cargoSha256 = "jIqLXlMA/UzEa72P15Pm2HTo9CtnPNZpdLeXoN0OVmU=";
  # cargoSha256 = nixpkgs.lib.fakeSha256;
  target = "x86_64-unknown-linux-gnu";
}
