{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
    unstable-nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    unstable-nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            unstable = import unstable-nixpkgs {
              inherit system;
              config.allowUnfree = true;
            };
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs;
          mkShellNoCC {
            # nativeBuildInputs is usually what you want -- tools you need to run
            nativeBuildInputs = with pkgs.buildPackages; [
              pkg-config
              openssl
            ];
            buildInputs = [
              lua
              unstable.lazygit
              llvmPackages.bintools
              eza
              unstable.neovim
              fd
              unstable.rustup
              rust-bin.stable.latest.default
            ];

            GREETING = "Environment is ready!";

            shellHook = ''
              exec fish --init-command '
              alias rm="roxide"
              alias find="fd"
              set -x PATH $PATH (set -q CARGO_HOME; and echo $CARGO_HOME; or echo ~/.cargo)/bin
              set -x PATH $PATH (set -q RUSTUP_HOME; and echo $RUSTUP_HOME; or echo ~/.rustup)/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin
              echo $GREETING | ${pkgs.lolcat}/bin/lolcat
              '
            '';
          };
      }
    );
}
