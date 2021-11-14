{
  description = "Repo - Repository management utility";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, flake-utils, fenix, naersk, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
        let
          pkgs = nixpkgs.legacyPackages.${system} // { inherit (fenix.packages.${system}.stable) cargo rustc rust-src clippy-preview rustfmt-preview; };

          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          version = manifest.package.version;

          naersk-lib = (
            naersk.lib."${system}".override {
              cargo = pkgs.cargo;
              rustc = pkgs.rustc;
            }
          );

          repo = naersk-lib.buildPackage {
            inherit version;
            pname = "repo";
            buildInputs = with pkgs; [ openssl ];
            nativeBuildInputs = with pkgs; [ pkg-config ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            root = ./.;
          };
        in
          rec {
            packages.repo = repo;
            defaultPackage = self.packages.${system}.repo;

            apps.repo = flake-utils.lib.mkApp { drv = packages.repo; };
            defaultApp = apps.repo;

            devShell = import ./shell.nix { inherit pkgs; };
          }
    );

}
