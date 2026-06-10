{
  description = "A Rust devShell example";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "hellorust";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              pkg-config
              rustToolchain
            ];
          };
      }
    );
}
