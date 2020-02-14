# { system ? builtins.currentSystem }:

let
  # pkgs = import <nixpkgs> { inherit system; };
  pkgs = import <nixpkgs> { };

  callPackage = pkgs.lib.callPackageWith pkgs;

  hoc = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildImage {
      name = "vbrandl/hits-of-code";
      tag = hoc.version;

      contents = [ pkg ];

      config = {
        Cmd = [ "/bin/hoc" ];
        WorkingDir = "/";
      };
    };

in dockerImage hoc
