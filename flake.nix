{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          overlays = [ inputs.rust-overlay.overlays.default ];
          src = craneLib.cleanCargoSource ./.;
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = [
            pkgs.opencv
            pkgs.libclang
          ];
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit
              src
              nativeBuildInputs
              buildInputs
              LD_LIBRARY_PATH
              ;
          };
          jec-subject = craneLib.buildPackage {
            inherit
              src
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              LD_LIBRARY_PATH
              ;
            strictDeps = true;

            doCheck = true;
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit
              src
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              LD_LIBRARY_PATH
              ;
            cargoClippyExtraArgs = "--verbose -- --deny warning";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit
              src
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              LD_LIBRARY_PATH
              ;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
            #programs.taplo.enable = true; # BUG: `nix flake check --all-systems` failed on aarch64-darwin
            programs.actionlint.enable = true;
            programs.mdformat.enable = true;

            settings = {
              excludes = [
                "LICENSE"
                ".gitignore"
                "Cargo.lock"
                "flake.lock"
              ];
            };
          };

          packages = {
            inherit jec-subject;
            default = jec-subject;
            doc = cargo-doc;
          };

          checks = {
            inherit
              jec-subject
              cargo-clippy
              cargo-doc
              ;
          };

          devShells.default = pkgs.mkShell rec {
            packages = [
              rust
              pkgs.pkg-config
              pkgs.nil
            ];

            buildInputs = [
              pkgs.opencv
              pkgs.libclang.lib
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
