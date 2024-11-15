{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system}; 
    manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  in {
    devShells.default = import ./nix/shell.nix {};
    packages.${system} = {
      ${manifest.name} = pkgs.callPackage ./nix/release/default.nix { };
      default = self.packages.${system}.${manifest.name};
    };
    nixosModules = {
      ${manifest.name} = ./nix/release/module.nix;
      nixosModules.default = self.nixosModules.${manifest.name};
    };
    homeManagerModules = {
      ${manifest.name} = ./nix/release/home-module.nix;
      default = self.homeManagerModules.${manifest.name};
    };
  };
}
