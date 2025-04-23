{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup
    pkg-config
    cmake
    openssl
    perl
    go
    clang
    llvmPackages.libclang
  ];
  # LIBCLANG_PATH = "${pkgs.libclang}";
  # Required for boring-sys
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.clang.version}/include";
  BORING_SSL_SOURCE_DIR = "${pkgs.boringssl}";
  BORING_SSL_IS_STATIC = "1";
}
