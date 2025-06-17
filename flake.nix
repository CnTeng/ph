{
  description = "Helper for impermanence.";

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

    treefmt = {
      url = "github:numtide/treefmt-nix";
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

      imports = [
        inputs.git-hooks-nix.flakeModule
        inputs.treefmt.flakeModule
      ];

      flake.nixosModules.default = import ./nix/module.nix self;

      perSystem =
        {
          config,
          pkgs,
          system,
          ...
        }:
        let
          toolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [
              "rust-src"
              "rustfmt"
              "rust-analyzer"
              "clippy"
            ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          _module.args = {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              (rust-bin.stable.latest.minimal.override {
                extensions = [
                  "rust-src"
                  "rustfmt"
                  "rust-analyzer"
                  "clippy"
                ];
              })
              cargo-edit
              cargo-sort

              config.treefmt.build.wrapper
            ];

            shellHook = config.pre-commit.installationScript;
          };

          checks.ph = import ./nix/check.nix self pkgs.nixosTest;

          packages = {
            ph = pkgs.callPackage ./nix/ph.nix {
              inherit rustPlatform;
            };
          };

          treefmt.programs = {
            nixfmt.enable = true;
            prettier.enable = true;
            rustfmt.enable = true;
            taplo.enable = true;
          };
        };
    };
}
