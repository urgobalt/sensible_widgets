{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
    in
      with pkgs; {
        devShells.default = mkShell {
          packages = [openssl dbus gtk4 gtk4-layer-shell pkg-config];
          buildInputs = [libpulseaudio];
          shellHook = ''
            export LD_LIBRARY_PATH=${dbus}/lib:${libpulseaudio}/lib
          '';
        };
      });
}
