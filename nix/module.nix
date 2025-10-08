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

  mkImpermanenceConfig =
    cfg:
    lib.optionalAttrs cfg.enable {
      directories = map (d: d.persistentStoragePath + d.dirPath) cfg.directories;
      files = map (f: f.persistentStoragePath + f.filePath) cfg.files;
    };

  mkPreservationConfig =
    cfg:
    let
      root = cfg.persistentStoragePath;
      userDirs = lib.flatten (map (u: u.directories) (lib.attrValues cfg.users));
      userFiles = lib.flatten (map (u: u.files) (lib.attrValues cfg.users));
      filter = builtins.filter (d: d.how != "_intermediate");
    in
    {
      directories = map (d: root + d.directory) (filter (cfg.directories ++ userDirs));
      files = map (f: root + f.file) (filter (cfg.files ++ userFiles));
    };

  deepMerge =
    lhs: rhs:
    lhs
    // rhs
    // (lib.mapAttrs (
      rName: rValue:
      let
        lValue = lhs.${rName} or null;
      in
      if lib.isAttrs lValue && lib.isAttrs rValue then
        deepMerge lValue rValue
      else if lib.isList lValue && lib.isList rValue then
        lib.unique (lValue ++ rValue)
      else
        rValue
    ) rhs);
in
{
  options.programs.ph = {
    enable = lib.mkEnableOption "ph";

    package = lib.mkPackageOption self.packages.${pkgs.system} "ph" { };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    environment.etc."ph/config.json".source =
      let
        impermanenceEnable = config.environment ? persistence;
        impermanenceConfig = lib.optionalAttrs impermanenceEnable (
          lib.mapAttrs (_: cfg: mkImpermanenceConfig cfg) config.environment.persistence
        );

        preservationEnable = config ? preservation && config.preservation.enable;
        preservationConfig = lib.optionalAttrs preservationEnable (
          lib.mapAttrs (_: cfg: mkPreservationConfig cfg) config.preservation.preserveAt
        );

      in
      format.generate "config.json" {
        persistence = deepMerge impermanenceConfig preservationConfig;
      };
  };
}
