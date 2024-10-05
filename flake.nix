{
  description = "Rust Environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true; # For clion
        overlays = [
          rust-overlay.overlays.default
        ];
      };

      # Includes cargo, rustc, rustfmt
      rust-stable = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src"]; # Include the rust stdlib source for intellij
      };
    in {
      devShell = pkgs.mkShell rec {
        name = "Rust Environment";

        nativeBuildInputs = with pkgs; [
          gcc14
          rust-stable
          # rust-analyzer # System install

          # Bevy Dependencies: https://github.com/bevyengine/bevy/blob/release-0.14.2/docs/linux_dependencies.md#nix
          pkg-config
        ];

        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          libxkbcommon
          wayland
        ];

        RUST_SRC_PATH = "${rust-stable}/lib/rustlib/src/rust/library";
        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
      };
    });
}
