{
  description = "Repo - Repository management utility";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          };

          inherit (pkgs) lib;
          craneLib = crane.lib.${system};

          repo = craneLib.buildPackage {
            src = ./.;
            buildInputs = with pkgs; [ openssl libiconv ]
              ++ (lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ]);
            nativeBuildInputs = with pkgs; [ pkg-config installShellFiles ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            preFixup = ''
              installManPage ./target/man/*
            '';
          };
        in
        rec {
          checks = { inherit repo; };

          apps = {
            repo = flake-utils.lib.mkApp {
              dev = repo;
            };
            default = apps.repo;
          };

          packages = {
            inherit repo;
            default = repo;
          };

          devShells.default = pkgs.mkShell {
            name = "repo";
            inputsFrom = [ repo ];
            nativeBuildInputs = with pkgs; [
              rustToolchain
            ];
          };
        });
}
