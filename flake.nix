{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        inherit system;
        pkgs = import nixpkgs { inherit system; };
        fpkgs = import fenix { inherit system; };
      });
    in
    {
      packages = forAllSystems
        ({ system, pkgs, fpkgs }:
          rec {
            default = tsunami;
            tsunami =
              pkgs.rustPlatform.buildRustPackage {
                pname = "tsunami";
                version = "0.1.0";
                cargoLock.lockFile = ./Cargo.lock;
                src = pkgs.lib.cleanSource ./.;
              };
          });

      devShell = forAllSystems ({ system, pkgs, fpkgs }:
        let
          ffpkgs = fpkgs.complete;
        in
        pkgs.mkShell {
          buildInputs = [
            ffpkgs.cargo
            ffpkgs.clippy
            ffpkgs.rust-src
            ffpkgs.rustc
            ffpkgs.rustfmt
            pkgs.wgo
          ];
        });

    };
}
