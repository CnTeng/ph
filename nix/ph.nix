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

  postInstall = ''
    installShellCompletion --cmd ph \
      --bash <($out/bin/ph completion bash) \
      --zsh <($out/bin/ph completion zsh) \
      --fish <($out/bin/ph completion fish)
  '';

  meta = {
    description = "Helper for impermanence.";
    homepage = "https://github.com/CnTeng/ph";
    license = lib.licenses.mit;
    mainProgram = "ph";
  };
}
