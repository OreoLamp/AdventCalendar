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

            # Environment variables
            shellHook = ''
                export RUST_SRC_PATH = ${toolchain}/lib/rustlib/src/rust/library
                export CARGO_HOME = $PWD/.cargo
                export CARGO_INSTALL_ROOT = $PWD/.cargo
                export CLIPPY_CONF_DIR = ./env
                export RUST_BACKTRACE = full
                export PATH $PATH:$PWD/.cargo/bin
            '';
        };
    };
}