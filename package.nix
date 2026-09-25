{
  lib,
  rustPlatform,
  makeWrapper,
  nix,
}:

rustPlatform.buildRustPackage {
  pname = "nix-juggler";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ makeWrapper ];

  postFixup = ''
    wrapProgram $out/bin/nix-juggler\
      --prefix PATH : ${lib.makeBinPath [ nix ]}
  '';

  meta = {
    description = "Manage nix pkgs in your config and have them available without having to rebuild";
    homepage = "https://github.com/BurningTurtle-dev/nix-juggler";
    license = lib.licenses.gpl3Only;
    mainProgram = "nix-juggler";
    platforms = lib.platforms.unix;
  };
}
