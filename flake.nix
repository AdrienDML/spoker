{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    wgsl-nix = {
        url = "github:AdrienDML/wgsl-nix";
        inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix, wgsl-nix}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # Rust setup.
        fenixPkgs = fenix.packages.${system};
        rust-toolchain = fenixPkgs.stable.toolchain;
        rust-env = [ rust-toolchain ] ++ (with pkgs; [
          rust-analyzer
          cargo-watch
          cargo-expand
        ]);

        wgsl-env = [wgsl-nix.packages.${system}.default]; 

        python-env = with pkgs; [
            python311
            python311Packages.matplotlib
            python311Packages.numpy
        ];

        buildInputs = with pkgs; [
          pkg-config
          udev 
          alsa-lib
          vulkan-loader
          vulkan-tools
          xorg.libX11
          xorg.libXcursor 
          xorg.libXi
          xorg.libXrandr # To use the x11 feature
          libxkbcommon
          wayland
        ];

        # Crane setup.
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

        # Source.
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src buildInputs;
        };

        # Deps.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        my-crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

      in
      {
        checks = {
          inherit my-crate;

          rust_fmt = craneLib.cargoFmt {
            inherit src;
          };

          rust_doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          rust_test = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = my-crate;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = rust-env ++ wgsl-env ++ [pkgs.just] ++ python-env;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs);
          WINIT_UNIX_BACKEND="x11";
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
