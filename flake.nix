{
  description = "A Helper for impermanence and preservation.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, nixpkgs, ... }:
    let
      forAllPkgs =
        f:
        nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (
          system: f nixpkgs.legacyPackages.${system}
        );
    in
    {
      nixosModules.default = import ./nix/module.nix self;

      packages = forAllPkgs (pkgs: {
        ph = pkgs.callPackage ./nix/package.nix { };
      });

      checks = forAllPkgs (pkgs: {
        ph = import ./nix/check.nix self pkgs.nixosTest;
      });

      devShells = forAllPkgs (
        pkgs:
        let
          rust-bin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
          toolchain = rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              extensions = [
                "rust-analyzer"
                "rust-src"
              ];
            }
          );
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              toolchain
              cargo-edit
              cargo-sort
            ];
          };
        }
      );

      formatter = forAllPkgs (
        pkgs:
        pkgs.nixfmt-tree.override {
          settings = {
            formatter.rustfmt = {
              command = "rustfmt";
              includes = [ "*.rs" ];
              options = [
                "--config"
                "skip_children=true"
              ];
            };

            global.excludes = [ "*.lock" ];
          };
        }
      );
    };
}
