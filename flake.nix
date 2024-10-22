{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flurry = {
      url = "github:itepastra/flurry";
    };
  };

  outputs = { self, fenix, nixpkgs, flurry, ... }:
    let
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        inherit system;
        inherit flurry;
        pkgs = import nixpkgs { inherit system; };
        fpkgs = import fenix { inherit system; };
      });
    in
    {
      packages = forAllSystems
        ({ pkgs, fpkgs, ... }:
          let
            toolchain = fpkgs.minimal.toolchain;
            fs = pkgs.lib.fileset;
          in
          rec {
            default = tsunami;
            tsunami =
              (pkgs.makeRustPlatform { cargo = toolchain; rustc = toolchain; }).buildRustPackage {
                pname = "tsunami";
                version = "0.1.0";
                cargoLock.lockFile = ./Cargo.lock;
                src = fs.toSource {
                  root = ./.;
                  fileset = fs.unions [
                    ./Cargo.lock
                    ./Cargo.toml
                    ./src
                    ./benches
                  ];
                };
              };
          });
      devShells = forAllSystems
        ({ pkgs, fpkgs, system, flurry, ... }:
          let
            ffpkgs = fpkgs.complete;
          in
          {
            default = pkgs.mkShell
              {
                buildInputs = [
                  ffpkgs.cargo
                  ffpkgs.clippy
                  ffpkgs.rust-src
                  ffpkgs.rustc
                  ffpkgs.rustfmt
                  pkgs.wgo
                  flurry.packages.${system}.flurry
                  self.packages.${system}.tsunami
                ];
              };
          });
      hydraJobs = {
        devShell.x86_64-linux = self.devShells.x86_64-linux.default;
        flurry.x86_64-linux = self.packages.x86_64-linux.flurry;
      };
    };
}
