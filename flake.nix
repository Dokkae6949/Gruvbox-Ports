{
  description = "Rust backend development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem = { system, pkgs, lib, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;

          overlays = [
            (import inputs.rust-overlay)
          ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust dependencies
            (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
            pkg-config
            openssl

            # Language Server
            rust-analyzer

            # Tooling
            rustfmt
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          # Shell hook to load .env file
          shellHook = ''
            if [ -f .env ]; then
              source .env
              echo "✓ .env file loaded"
            fi
          '';
        };
      };
    };
}
