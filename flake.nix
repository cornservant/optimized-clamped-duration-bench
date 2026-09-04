{
  description = "alarm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/c5c4a43b0e8056328ec4529f735cabdb8f1942bb";
  };

  outputs =
    { nixpkgs, ... }:
    let
      eachSupportedSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
      eachPkgs = f: eachSupportedSystem (system: f nixpkgs.legacyPackages.${system});
    in
    builtins.mapAttrs (_: eachPkgs) {
      formatter = pkgs: pkgs.nixfmt;
      devShells = pkgs: {
        default = pkgs.mkShell {
          LD_LIBRARY_PATH =
            pkgs.lib.makeLibraryPath [
              pkgs.julia
            ]
            + ":${pkgs.julia}/lib/julia";
          packages = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.rustfmt
            pkgs.clippy
            pkgs.julia
          ];
        };
      };
    };
}
