{ sources ? import ./nix/sources.nix
, system ? builtins.currentSystem
}:

let
  rustOverlay = import "${sources.nixpkgs-mozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${sources.cargo2nix}/overlay";

  pkgs = import sources.nixpkgs {
  # pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "stable";
    packageFun = import ./Cargo.nix;
    localPatterns =
      [
        ''^(src|tests)(/.*)?''
        ''[^/]*\.(rs|toml)$''
        # include other directory from the project repository
        ''^templates(/.*)?''
        ''^static(/.*)?''
        ''^.git.*(/.*)?''
      ];
    # packageOverrides
  };
in
  (rustPkgs.workspace.hoc {}).overrideAttrs (drv: {
      buildInputs = drv.buildInputs or [ ] ++ [ pkgs.git ];
  })
