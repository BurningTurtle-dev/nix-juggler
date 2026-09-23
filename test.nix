{ pkgs, ... }:
{
home.packages = with pkgs; [
+
blender
kdePackages.dolphin
}
];
}