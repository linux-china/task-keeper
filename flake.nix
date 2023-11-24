{
  description = "task-keeper";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    # rust-overlay.url = "github:oxalica/rust-overlay";
    fenix = {
      url = "github:nix-community/fenix"; # needed for devenv's languages.rust
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs@{ self, flake-parts, nixpkgs, naersk, devenv, fenix, ... }: (
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, lib, ... }: (
        let

          # Naersk example from: https://github.com/nix-community/naersk/blob/master/examples/static-musl/flake.nix
          rustToolchain = with fenix.packages.${system}; combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-unknown-linux-musl.latest.rust-std
          ];
          naerskLib = naersk.lib.${system}.override {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };

          task-keeper = naerskLib.buildPackage {
            # https://crane.dev/getting-started.html
            src = ./.;
            # set binary name for nix run (tries `task-keeper`) - https://github.com/nix-community/naersk/issues/127
            meta.mainProgram = "tk";

            # CARGO_BUILD_TARGET = "wasm-unknown-unknown";
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";

            # Add extra inputs here or any other derivation settings
            # buildInputs = [];
            nativeBuildInputs = with pkgs; [ pkgsStatic.stdenv.cc ];

            # doCheck = true; - TODO: needs all deps in build inputs
          };
        in
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          checks = {
            inherit task-keeper;
          };

          packages.default = task-keeper;

          devenv.shells.default = (import ./devenv.nix { inherit pkgs; });
          devenv.shells.test = (import ./devenv-test.nix { inherit pkgs; });
        }
      );
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    }
  );
}
