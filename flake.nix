{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages = rec {
          hyprland-display-tools = naersk-lib.buildPackage {
            name = "hyprland-display-tools";
            version = "0.1.0";
            src = ./.;
          };
          default = hyprland-display-tools;
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
