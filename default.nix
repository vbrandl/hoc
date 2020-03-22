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
rec {
  inherit rustPkgs;
  shell = pkgs.mkShell {
    inputsFrom = pkgs.lib.mapAttrsToList (_: pkg: pkg { }) rustPkgs.noBuild.workspace;
    nativeBuildInputs = with rustPkgs; [ cargo rustc ];
  };
  package = (rustPkgs.workspace.hoc {}).overrideAttrs (drv: {
      buildInputs = drv.buildInputs or [ ] ++ [ pkgs.git ];
  });
  dockerImage =
    pkgs.dockerTools.buildImage {
      name = "vbrandl/hits-of-code";
      tag = package.version;

      contents = [ package ];

      config = {
        Cmd = [ "/bin/hoc" ];
        WorkingDir = "/";
      };
    };
}
