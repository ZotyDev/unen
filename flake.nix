{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

    completeToolchain = toolchain.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
        "clippy"
        "rustfmt"
      ];
    };
  in
  {
    devShells.${system}.default = pkgs.mkShell rec {
      packages = [
        completeToolchain
      ];

      RUST_SRC_PATH = "${completeToolchain}/lib/rustlib/src/rust/library";

      buildInputs = with pkgs; [
        pkg-config
        libxkbcommon
        vulkan-loader
        wayland
      ];

      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}"
      '';
    };
  };
}
