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
    rustfmt
    clippy
    mdbook
    cargo-bloat
    libiconv # needed for git2 (libgit2)
  ] ++ (lib.optionals pkgs.stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ]);

  CARGO_BUILD_RUSTFLAGS = if pkgs.stdenv.isDarwin then "-C rpath" else null;
  RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
}
