{
  rustPlatform,
  version ? "git",
  lib,
  installShellFiles,

}:
let
  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.toml
      ../Cargo.lock
    ];
  };
in
rustPlatform.buildRustPackage rec {
  pname = "ph";
  inherit version src;

  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  nativeBuildInputs = [
    installShellFiles
  ];

  postInstall = '''';

  meta = {
    description = "Helper for impermanence.";
    homepage = "https://github.com/CnTeng/ph";
    license = lib.licenses.mit;
    mainProgram = "ph";
  };
}
