{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    hello
    openssl.dev

    # keep this line if you use bash
    bashInteractive
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
}
