self:
{
  lib,
  config,
  pkgs,
  ...
}:
let
  cfg = config.programs.ph;
in
{
  options.programs.ph = {
    enable = lib.mkEnableOption "ph";

    package = lib.mkPackageOption self.packages.${pkgs.system} "ph" { };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
