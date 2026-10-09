# nix-juggler

## What is it?

A cli app that offers a pacman like way to install and add pkgs to a nix config. It installs the pkg via "nix profile" and adds it to a nix module. The nix profile version is replaced durring the rebuild of the config.

## Installation

nix-juggler needs to be installed as a nix flake.

In flake.nix:

```nix
{
  # ...
  inputs = {
    nix-juggler.url = "github:BurningTurtle-dev/nix-juggler";
      # ...
  };
}

```
