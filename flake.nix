{
  description = "A markdown script that searches all notes and syncs the note cards with ankiconnect";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        bi = with pkgs; [
          cargo
          rustc
          rustfmt

          pkg-config
          stdenv.cc.libc
          clang

          openssl
        ];
        clang_path = "${pkgs.llvmPackages.libclang.lib}/lib";
        dev_tools = with pkgs; [
          postman
        ];
      in {
        devShell = pkgs.mkShell {
          buildInputs = bi ++ dev_tools;
          LIBCLANG_PATH = clang_path;
        };
      }
    );
}
