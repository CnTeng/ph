self:
{
  lib,
  config,
  pkgs,
  ...
}:
let
  cfg = config.programs.ph;
  format = pkgs.formats.json { };

  mkPersistNode =
    name: value:
    lib.optionalAttrs config.environment.persistence.${name}.enable {
      directories = map (
        dir: dir.persistentStoragePath + dir.dirPath
      ) config.environment.persistence.${name}.directories;
      files = map (
        file: file.persistentStoragePath + file.filePath
      ) config.environment.persistence.${name}.files;
    };
in
{
  options.programs.ph = {
    enable = lib.mkEnableOption "ph";

    package = lib.mkPackageOption self.packages.${pkgs.system} "ph" { };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    environment.etc."ph/config.json".source = format.generate "config.json" {
      persistence = lib.mapAttrs (name: value: mkPersistNode name value) config.environment.persistence;
    };
  };
}
