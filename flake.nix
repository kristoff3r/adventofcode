{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      my-rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
        targets = [
          "x86_64-unknown-linux-gnu"
          "wasm32-unknown-unknown"
        ];
      };
      load-aoc = pkgs.writeScriptBin "aoc-load" ''
        if [ $1 ]
        then
            URL="https://adventofcode.com/$1/day/$2/input"
        else
            URL=`date +https://adventofcode.com/%Y/day/%-d/input`
        fi
        echo $URL $AOC_COOKIE
        ${pkgs.curl}/bin/curl --cookie "session=$AOC_COOKIE" $URL > in.txt
      '';
    in
    {
      devShell."${system}" = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs; [
          load-aoc
          my-rust
          clang
          lld
          udev
          alsa-lib
          vulkan-loader
          pkgconfig
          vulkan-tools
          shaderc
          renderdoc
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          wgpu-utils
        ];

        VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
        LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
          udev
          alsa-lib
          vulkan-loader
        ];
      };
    };
}
