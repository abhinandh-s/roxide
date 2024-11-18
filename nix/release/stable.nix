# wip
{ pkgs ? import <nixpkgs> { } }:
let 
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
  pkgs.stdenv.mkDerivation rec {
    pname = manifest.name;
    version = manifest.version;
    src = pkgs.fetchFromGithub {
      url = "https://github.com/abhi-xyz/${pname}/releases/download/v0.1.9/${pname}-x86_64-unknown-linux-gnu.tar.gz";
      sha256 = "";
    };
    meta = with pkgs.lib; {
      description = manifest.description;
      homepage = "https://github.com/abhi-xyz/${pname}";
      changelog = "https://github.com/abhi-xyz/${pname}/releases";
      license = licenses.mit;
      maintainers = with maintainers; [ Abhinandh S ];
      platforms = platforms.linux;
      mainProgram = manifest.name;
    };
  }
