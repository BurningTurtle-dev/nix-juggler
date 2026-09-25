{
  lib,
  rustPlatform,
  makeWrapper,
  nix,
}:

rustPlatform.buildRustPackage rec {
  pname = "nix-juggler";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ makeWrapper ];

  # nix-juggler-client shells out to `nix profile add/remove/list` at runtime,
  # so make sure `nix` is on PATH regardless of the caller's environment.
  # nix-juggler-writer is spawned by the client via its own exe path
  # (exe.with_file_name), so it just needs to live alongside the client in
  # $out/bin, which buildRustPackage already gives us for free.
  postFixup = ''
    wrapProgram $out/bin/nix-juggler-client \
      --prefix PATH : ${lib.makeBinPath [ nix ]}
  '';

  meta = {
    description = "Manage nix pkgs in your config and have them available without having to rebuild";
    homepage = "https://github.com/BurningTurtle-dev/nix-juggler";
    license = lib.licenses.gpl3Only;
    mainProgram = "nix-juggler-client";
    platforms = lib.platforms.unix;
  };
}
