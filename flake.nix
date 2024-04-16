{
  description = "package definition and devshell for Boop-GTK";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";
  inputs.devshell.inputs.flake-utils.follows = "flake-utils";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "github:nix-community/flake-compat";
  inputs.flake-compat.flake = false;

  outputs = {
    nixpkgs,
    advisory-db,
    crane,
    devshell,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [devshell.overlays.default];
      };

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-e4mlaJehWBymYxJGgnbuCObVlqMlQSilZ8FljG9zPHY=";
      };

      librusty_v8 = pkgs.callPackage ./librusty_v8.nix {};

      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      gladeFilter = path: builtins.match ".*glade$" path != null;
      svgFilter = path: builtins.match ".*svg$" path != null;
      embedFilter = path: builtins.match ".*lang$" path != null;
      jsFilter = path: builtins.match ".*js$" path != null;
      filter = path: type: (gladeFilter path) || (svgFilter path) || (embedFilter path) || (jsFilter path) || (craneLib.filterCargoSources path type);

      commonArgs = {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          inherit filter;
        };
        buildInputs = with pkgs; [atk cairo gdk-pixbuf gtk3 gtksourceview3 harfbuzz pango zlib];
        nativeBuildInputs = with pkgs; [glib pkg-config];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        RUSTY_V8_ARCHIVE = librusty_v8;
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {doCheck = false;});

      Boop-GTK = craneLib.buildPackage (commonArgs // {doCheck = false;});
      Boop-GTK-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
        });
      Boop-GTK-fmt = craneLib.cargoFmt (commonArgs // {});
      Boop-GTK-audit = craneLib.cargoAudit (commonArgs // {inherit advisory-db;});
      Boop-GTK-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit cargoArtifacts;
          src = ./.;
          partitions = 1;
          partitionType = "count";
        });
    in {
      checks = {
        inherit Boop-GTK Boop-GTK-audit Boop-GTK-clippy Boop-GTK-fmt Boop-GTK-nextest;
      };

      packages.default = Boop-GTK;

      apps.default = flake-utils.lib.mkApp {drv = Boop-GTK;};

      devShells.default = let
        inputsOf = drv:
          (drv.buildInputs or [])
          ++ (drv.nativeBuildInputs or [])
          ++ (drv.propagatedBuildInputs or [])
          ++ (drv.propagatedNativeBuildInputs or []);
      in
        pkgs.devshell.mkShell {
          imports = ["${pkgs.devshell.extraModulesDir}/language/c.nix"];
          bash = {interactive = "";};

          language.c = {
            includes = inputsOf Boop-GTK;
            compiler = pkgs.stdenv.cc;
          };

          env = [
            {
              name = "DEVSHELL_NO_MOTD";
              value = 1;
            }
          ];

          packages = with pkgs; [
            cargo-edit
            cargo-nextest
            cargo-release
            rustStable
            stdenv.cc
          ];

          packagesFrom = [Boop-GTK];
        };
    });
}
