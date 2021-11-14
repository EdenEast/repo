{ pkgs ? import <nixpkgs> {
    overlays = [
      (
        import (
          fetchTarball
            "https://github.com/nix-community/fenix/archive/main.tar.gz"
        )
      )
      (
        self: super: {
          rustc = super.fenix.stable.rustc;
          cargo = super.fenix.stable.cargo;
          rust-src = super.fenix.stable.rust-src;
        }
      )
    ];
  }
}:

pkgs.mkShell {
  name = "repo";
  packages = with pkgs; [
    rustc
    cargo
    rust-analyzer
    pkg-config
    openssl
    rustfmt-preview
    clippy-preview
    mdbook
  ];

  RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
}