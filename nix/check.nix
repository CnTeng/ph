self: nixosTest:
nixosTest {
  name = "ph";
  nodes.machine = {
    imports = [ self.nixosModules.default ];
    programs.ph.enable = true;
  };

  testScript = ''
    machine.wait_for_unit("default.target")
    machine.succeed("which ph")
  '';
}
