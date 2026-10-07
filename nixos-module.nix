{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.nix-juggler;
  tomlFormat = pkgs.formats.toml { };
in
{
  options.programs.nix-juggler = {
    enable = lib.mkEnableOption "nix-juggler, manage nix profile pkgs without rebuilding";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "The nix-juggler package to install.";
    };

    nixModulePath = lib.mkOption {
      type = lib.types.str;
      description = ''
        Path to the nix module file nix-juggler manages (the file containing
        your `home.packages = with pkgs; [ ... ];` block).
      '';
      example = "/etc/nixos/juggler.nix";
    };

    pkgSource = lib.mkOption {
      type = lib.types.str;
      default = "nixpkgs";
      description = "Flake reference used as the source when installing packages.";
    };

    writerPrefix = lib.mkOption {
      type = lib.types.enum [
        "sudo"
        "doas"
        ""
      ];
      default = "";
      description = ''
        Command used to run the writer with elevated privileges.
        Leave as "" to run unprivileged. If set to "sudo" or "doas", that
        program must already be installed and available on PATH yourself,
        nix-juggler does not install or manage it.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    environment.etc."juggler.toml".source = tomlFormat.generate "juggler.toml" {
      nix_module_path = cfg.nixModulePath;
      pkg_source = cfg.pkgSource;
      writer_prefix = cfg.writerPrefix;
    };
  };
}
