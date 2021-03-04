let
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ oxalica_overlay ]; };
  rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;

in
with nixpkgs;
pkgs.mkShell {
  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    rust_channel
    pkgconfig
  ];

  RUST_BACKTRACE = 1;
}
