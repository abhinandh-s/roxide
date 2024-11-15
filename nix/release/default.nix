{ pkgs ? import <nixpkgs> { } }:
let 
  rustPlatform = pkgs.rustPlatform;
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
  rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ../../Cargo.lock;
    src = pkgs.lib.cleanSource ../../.;
    doCheck = false;

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
