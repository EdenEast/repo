{
  description = "Repo - Repository management utility";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;
        craneLib = crane.lib.${system};

        repo = craneLib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [ openssl libiconv ]
            ++ (lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ]);
          nativeBuildInputs = with pkgs; [ pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
      rec {
        packages.default = repo;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ repo ];
          nativeBuildInputs = with pkgs; [
            # Core rust
            cargo
            rustc

            # Development tools
            rustfmt
            clippy
          ];

          RUST_SRC_PATH = "${rust-src}/rustlib/src/rust/library";
        };
      });
}
