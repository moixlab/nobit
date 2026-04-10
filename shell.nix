{ pkgs ? import <nixpkgs> {
    overlays = [ (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" ];
    targets = [];
  };
in
pkgs.mkShell {
    # Herramientas
    nativeBuildInputs = with pkgs; [
        rustToolchain # Rust
        sqlite # SQL
    ];
    # Librerías
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
    # Configuración
    shellHook = ''
      export PATH="$HOME/.cargo/bin:$PATH"

      cargo install --locked just
    '';
}