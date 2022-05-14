{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        SDL2
        SDL2_image
        SDL2_gfx
        SDL2_mixer
        SDL2_ttf
        rustup
        gcc
        pkg-config
    ];
    buildInputs = with pkgs; [ ];
    inputsFrom = with pkgs; [ ];
    hardeningDisable = [ "all" ];
    shellHook = "";
    
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
        "/run/opengl-driver"
        "/run/opengl-driver-32"
    ];
}


