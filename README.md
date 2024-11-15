# roxide

`roxide` is a lightweight, high-performance alternative to the traditional `rm` command, designed with an emphasis on speed and data preservation. Unlike standard rm, which permanently deletes files, rid safely moves them to a designated "`trash`" directory. This approach enables efficient and fast file removal without risking unintended data loss.

## Getting started

![Tutorial][tutorial]


## Features
- `rid` _filename_ : files will moved to `trash dir`. ie, $HOME/.local/share/Trash/files/
- `rid --revert` : restors perviously removed file/directory


## Installation

## Build

```bash
git clone https://github.com/abhi-xyz/rid.git --depth=1 
cd rid
cargo build --release
cp target/release/rid /usr/local/bin/
```


### Nixos

Add `rid` as a flake input: In your flake.nix file, add the rid input pointing to its GitHub repository.
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rid = {
      url = "github:abhi-xyz/rid";
      inputs.nixpkgs.follows = "nixpkgs"; # Follows the nixpkgs input
    };
  };

  outputs = { self, rid, nixpkgs, ... }@inputs:
  {
    # Your NixOS configuration or other settings
  };
}
```

Include rid in NixOS configuration: In the outputs section, set up nixosConfigurations and add rid as a module to enable it within your system configuration.
```nix
{
  outputs = { self, rid, nixpkgs, ... }@inputs:
  {
    nixosConfigurations."my-nixos-host" = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = { inherit inputs; };
      modules = [
        ./hosts/configuration.nix # Main NixOS configuration file
        rid.nixosModules.rid      # Enable the "rid" module from the flake
      ];
    };
  };
}
```
Activate "rid" in configuration.nix: Within configuration.nix, enable the "rid" program:
```nix
{
  # Other configurations...
  
  program.rid.enable = true; # note: its program not programs
}
```
Apply Configuration: After updating your configuration, apply it with:
```bash
sudo nixos-rebuild switch --flake .
```

## known bugs
- ~~golb pattern isn't working~~

## Planned Features
- rid --revert:  Restores files from the trash directory to their original location.
- rid --gc <TIME_PERIOD>:  Performs garbage collection on trashed files, automatically clearing files older than a specified period.
- rid -f:  Forces deletion without moving files to the trash directory.
- rid -s:  Shreds files for secure deletion.
