# flake.nix
{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = {
        self,
        nixpkgs,
        rust-overlay,
    }: let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
            inherit system;
            overlays = [rust-overlay.overlays.default];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./env/toolchain.toml;
    in {
        devShells.${system}.default = pkgs.mkShell {
            
            # Packages (duh)
            packages = [
                toolchain

                # Unwrapped version because the guide told me i want it
                pkgs.rust-analyzer-unwrapped
            ];

            # Env variables (also duh)
            env = {
                CARGO_MANIFEST_DIR = "./env";
                CLIPPY_CONF_DIR = "./env";
                RUST_BACKTRACE = "full";
            };
        };
    };
}
