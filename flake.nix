{
  description = "prisma-to-sea-orm development environment";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = { pkgs, system, ... }: {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
              (final: prev: {
                rustToolchain = final.rust-bin.nightly.latest.default.override {
                  extensions = ["rust-src" "rust-analyzer"];
                };
              })
            ];
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              nodejs
              pnpm
            ];

            shellHook = ''
              echo "prisma-to-seaorm dev shell"
              echo "  Rust: $(rustc --version)"
              echo "  Node: $(node --version)"
              echo "  pnpm: $(pnpm --version)"
            '';
          };
        };
    };
}
