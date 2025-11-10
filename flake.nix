{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    fenix,
    nixpkgs,
    flake-utils,
    ...
  }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./toolchain.toml;
        sha256 = "sha256-2eWc3xVTKqg5wKSHGwt1XoM/kUBC6y3MWfKg74Zn+fY=";
      };
      buildInputs = with pkgs; [
        pkg-config
        libxkbcommon
        vulkan-loader
        wayland
      ];
    in {
      devShells.default = pkgs.mkShell {
        packages = [
          toolchain
        ] ++ buildInputs;
        shellHook = ''
          echo "rustc $(rustc --version)"
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}"
        '';
      };
    });
}
