{ pkgs, ... }:
{
home.packages = with pkgs; [
+
blender
btop
kdePackages.dolphin
}
];
}