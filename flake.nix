{
  description = "A Helper for impermanence and preservation.";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: f nixpkgs.legacyPackages.${system});

      mkRustfmt = pkgs: pkgs.rustfmt.override { asNightly = true; };
    in
    {
      packages = forEachSystem (pkgs: {
        ph = pkgs.callPackage ./nix/package.nix { };
      });

      nixosModules.default = import ./nix/module.nix self;

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            cargo-edit
            cargo-sort
            clippy
            rust-analyzer
            (mkRustfmt pkgs)
          ];
        };
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
        pkgs.nixfmt-tree.override {
          settings.formatter.rustfmt = {
            command = "rustfmt";
            options = [
              "--config"
              "skip_children=true"
            ];
            includes = [ "*.rs" ];
          };
          runtimeInputs = [ (mkRustfmt pkgs) ];
        }
      );
    };
}
