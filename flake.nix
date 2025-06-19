{
  description = "A Helper for impermanence and preservation.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      imports = [ inputs.git-hooks-nix.flakeModule ];

      flake.nixosModules.default = import ./nix/module.nix self;

      perSystem =
        { config, pkgs, ... }:
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
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              toolchain
              cargo-edit
              cargo-sort
            ];
            shellHook = config.pre-commit.installationScript;
          };

          packages.ph = pkgs.callPackage ./nix/package.nix { };

          checks.ph = pkgs.nixosTest {
            name = "ph";
            nodes.machine = {
              imports = [ self.nixosModules.default ];
              programs.ph.enable = true;
            };
            testScript = ''
              machine.wait_for_unit("default.target")
              machine.succeed("which ph")
            '';
          };

          formatter = pkgs.nixfmt-tree.override {
            settings.formatter.rustfmt = {
              command = "rustfmt";
              includes = [ "*.rs" ];
              options = [
                "--config"
                "skip_children=true"
              ];
            };
          };

          pre-commit.settings.hooks = {
            treefmt = {
              enable = true;
              package = config.formatter;
            };
            commitizen.enable = true;
          };
        };
    };
}
