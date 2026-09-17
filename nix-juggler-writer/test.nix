{ pkgs, ... }:
{
  home.packages = with pkgs; [
    obsidian
    openscad
    qalculate-gtk
    signal-desktop
    calibre
  ];
}
