{
  description = "Entanglement";

  nixConfig = {
    extra-substituters = [
      "https://projects.cache.profidev.io"
    ];

    extra-trusted-public-keys = [
      "profidev.cachix.org:tg4xEn64UMdvA5jJYT8omo/CQHk8+spLyeGT2YAku70="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      nix-filter,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustBuildInputs = with pkgs; [
          pkg-config
          glib
          at-spi2-atk
          libsoup_3
          webkitgtk_4_1
          xdotool
          perl
          tailwindcss_4
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "entanglement";
          version = "0.1.0";

          src = nix-filter {
            root = ./.;
            include = [
              (nix-filter.lib.inDirectory "app")
              "backend/Cargo.toml"
              "backend/entity/Cargo.toml"
              "backend/migration/Cargo.toml"
              "Cargo.toml"
              "Cargo.lock"
            ];
          };

          buildInputs = rustBuildInputs;

          nativeBuildInputs =
            with pkgs;
            [
              dioxus-cli
              makeWrapper
              wrapGAppsHook4
            ]
            ++ rustBuildInputs;

          buildAndTestSubdir = "app";

          buildPhase = ''
            runHook preBuild
            cd app
            dx build --release --linux
            cd ..
          '';

          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin/assets
            cp target/dx/entanglement/release/linux/app/entanglement $out/bin/
            cp -r target/dx/entanglement/release/linux/app/assets/* $out/bin/assets/
          '';

          postInstall = ''
            wrapGAppsHook
            # Required for the app to find its assets
            wrapProgram $out/bin/entanglement \
              --chdir $out/bin
          '';

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "dioxus-attributes-0.1.0" = "sha256-tI26vv7fvNR18KsUJvBTXZ0c7Wc/63Qq88NAWuWMoHs=";
            };
          };

          doCheck = false;
        };
      }
    );
}
