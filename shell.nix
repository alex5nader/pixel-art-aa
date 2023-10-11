{ pkgs ? import <nixpkgs> { } }:

let
  inherit (pkgs) lib;
in

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    pkg-config
    mold
    clang
  ];
  buildInputs = with pkgs; [
    udev
    alsa-lib
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi # To use x11 feature
    # xorg.libXft
    # libxkbcommon wayland # To use wayland feature
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

