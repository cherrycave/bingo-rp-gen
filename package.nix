{
  rustPlatform,
  lib,
}:

rustPlatform.buildRustPackage {
  pname = "bingo_texture_gen";
  version = (lib.importTOML ./Cargo.toml).package.version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./base_pack
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    mainProgram = "bingo_texture_gen";
  };
}
