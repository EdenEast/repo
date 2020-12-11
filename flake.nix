# Example resources:
#   - https://github.com/Stupremee/nix/blob/bc76927/templates/rust/flake.nix
{
  description = "locations - find your friends";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs, flake-compat }:
    utils.lib.eachDefaultSystem (system:
      let
        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = manifest.package.version;
        pname = "repo";

        pkgs = nixpkgs.legacyPackages."${system}";

        # get our specific rust version
        mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") { };
        rustChannel = mozilla.rustChannelOf {
          # rust version 1.48.0
          channel = "stable";
          date = "2020-11-19";
          # SHA256 of latest stable from https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256
          sha256 = "ef3b7eac7671c7e85ae0ffd49d3d9a348b81b633e47a3548d8fc9c02df80a62c";
        };
        rust = rustChannel.rust;

        # override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          # Currently naersk requires the nightly version of cargo as `--out-dir` flag is unstable
          # see: https://github.com/nmattia/naersk/issues/100 for more information
          # cargo = rust;
          rustc = rust;
        };

        buildInputs = with pkgs; [ openssl pkgconfig ];

      in rec {
        # `nix build`
        defaultPackage = packages."${pname}";
        packages."${pname}" = naersk-lib.buildPackage {
          inherit pname version buildInputs;
          root = ./.;
          copyBins = true;
          copylibs = false;
          release = true;
        };

        # `nix run`
        apps."${pname}" = utils.lib.mkApp { drv = packages."${pname}"; };

        defaultApp = apps."${pname}";

        # `nix develop`
        devShell = pkgs.mkShell {
          # supply the specific rust version
          nativeBuildInputs = with pkgs; [ rust ];
        };
      });
}
