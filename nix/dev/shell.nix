{ pkgs }:

with pkgs; mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs.buildPackages; [ lua ];
  buildInputs = [
    openssl
    pkg-config
    llvmPackages.bintools
    eza
    unstable.neovim
    fd
    unstable.rustup
    rust-bin.stable.latest.default
  ];

  shellHook = ''
    alias ls=eza
    alias find=fd
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    echo "Environment ready!" | ${pkgs.lolcat}/bin/lolcat
    '';
}
