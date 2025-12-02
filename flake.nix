{
  description = "Advent of Code 2025";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Select latest stable Rust with vital extensions
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })

            # Essential tools
            pkg-config
            openssl # Often needed if you use 'reqwest' or web crates
            
            # AoC Quality of Life
            cargo-watch # Re-runs code when you save: cargo watch -x run
            cargo-edit  # Adds commands like: cargo add serde
          ];

          # Environment variables to help tools find the SSL certs
          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
          '';
        };
      }
    );
}