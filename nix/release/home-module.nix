{
config,
pkgs,
lib,
...
}:
let
    manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
  {
  options.program.${manifest.name} = {
    enable = lib.mkEnableOption "Enable the program";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./nix/release/default.nix { };
      description = "The package to use.";
    };
  };

  config = lib.mkIf config.program.${manifest.name}.enable {
    home.packages = [ config.program.${manifest.name}.package ];
  };
}
