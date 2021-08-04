# Example resources:
#   - https://github.com/Stupremee/nix/blob/bc76927/templates/rust/flake.nix
{
  description = "Repo - Repository management utility";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, flake-compat }:
    utils.lib.eachDefaultSystem (
      system:
        let
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          version = manifest.package.version;
          pname = "repo";

          pkgs = nixpkgs.legacyPackages."${system}";

          naersk-lib = naersk.lib."${system}";
        in
          rec {
            # `nix build`
            defaultPackage = packages."${pname}";
            packages."${pname}" = naersk-lib.buildPackage {
              inherit pname version;
              root = ./.;
              copyBins = true;
              copylibs = false;
              release = true;

              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = [ pkgs.openssl ];
            };

            # `nix run`
            apps."${pname}" = utils.lib.mkApp { drv = packages."${pname}"; };
            defaultApp = apps."${pname}";

            # `nix develop`
            devShell = pkgs.mkShell {
              name = "${pname}";
              nativeBuildInputs = with pkgs; [ rustc cargo ];
              buildInputs = with pkgs; [ openssl pkg-config ];
            };
          }
    );
}
