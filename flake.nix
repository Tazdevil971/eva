{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
    flake-compat = {
      url = "github:NixOS/flake-compat";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      fenix,
      ...
    }:
    {
      overlays.default = final: prev: {
        eva-toolchain = import ./nix/toolchain.nix { pkgs = final; };
      };
    }
    // utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
            self.overlays.default
          ];
        };
      in
      {
        devShells.default = import ./nix/shell.nix { inherit pkgs; };
        packages = rec {
          default = eva-toolchain.normal;
          inherit (pkgs) eva-toolchain;
        };
      }
    );
}
