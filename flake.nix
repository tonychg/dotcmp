{
  description = "GuineaPigsClub Staking API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rust-bin.stable.latest.default
            rust-analyzer
            pkg-config
            openssl
          ];

          buildInputs = with pkgs; [
            sops
            go-task
            kubectl
            kustomize
            docker-compose
            clippy
            sqlx-cli
            pgcli
            postgresql
            sqlint
            sqlfluff
            husky
          ];
        };
      }
    );
}
