{ config, pkgs, lib, ... }:
let
  tomlFormat = pkgs.formats.toml { };
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
  {
  options.program.${manifest.name} = {
    enable = lib.mkEnableOption "Enable the ${manifest.name} program";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./default.nix { };
      description = "The ${manifest.name} package to use.";
    };
    settings = lib.mkOption {
      type = tomlFormat.type;
      default = { };
      example = lib.literalExpression ''
        [test-01]
        one = "test one"
      '';
      description = ''
      Configuration written to {file}`$XDG_CONFIG_HOME/${manifest.name}/config.toml`.
      '';
    };
  };
  config = lib.mkIf config.program.${manifest.name}.enable {
    environment.systemPackages = [ config.program.${manifest.name}.package ];
    # cat /run/current-system/etc/${manifest.name}/config.toml
    environment.etc."${manifest.name}/config.toml" = lib.mkIf (config.program.${manifest.name}.settings != { }) {
      source = tomlFormat.generate "config.toml" config.program.${manifest.name}.settings;
    };
  };
}
