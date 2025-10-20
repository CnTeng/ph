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
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: f nixpkgs.legacyPackages.${system});

      mkRustToolchain =
        pkgs:
        let
          rust-bin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
        in
        rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      nixosModules.default = import ./nix/module.nix self;

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            (mkRustToolchain pkgs)
            cargo-edit
            cargo-sort
          ];
        };
      });

      packages = forEachSystem (pkgs: {
        ph = pkgs.callPackage ./nix/package.nix { };
      });

      checks = forEachSystem (pkgs: {
        ph = pkgs.nixosTest {
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
      });

      formatter = forEachSystem (
        pkgs:
        (pkgs.nixfmt-tree.override {
          settings.formatter.rustfmt = {
            command = "${mkRustToolchain pkgs}/bin/rustfmt";
            includes = [ "*.rs" ];
            options = [
              "--config"
              "skip_children=true"
            ];
          };
        })
      );
    };
}
