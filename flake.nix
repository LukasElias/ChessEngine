{
  description = "LukasElias's ChessEngine in a flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem = {
        self',
        pkgs,
        ...
      }: let
        craneLib = inputs.crane.mkLib pkgs;
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        treefmt.programs = {
          alejandra.enable = true;
          rustfmt.enable = true;
        };
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            rust-analyzer
            nil
          ];
        };
        packages.default =
          craneLib.buildPackage
          (commonArgs
            // {
              inherit cargoArtifacts;
            });
        apps.default = {
          type = "app";
          program = pkgs.writeShellApplication {
            name = "chess-tui";
            runtimeInputs = [
              pkgs.chess-tui
              self'.packages.default
            ];
            text = ''
              ${pkgs.chess-tui}/bin/chess-tui -e ${self'.packages.default}/bin/chess_engine
            '';
          };
        };
      };
    };
}
