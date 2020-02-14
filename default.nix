# { system ? builtins.currentSystem }:
{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { }
, callPackage ? pkgs.callPackage
}:

let
  cargoNix = callPackage ./Cargo.nix { };

  hoc = cargoNix.rootCrate.build;

  buildInputs = [ pkgs.openssl pkgs.cacert ];

  # version = hoc.version;

# in hoc.rootCrate.build
in
pkgs.symlinkJoin {
  name = hoc.name;
  version = hoc.crateVersion;
  paths = [ hoc ];

  buildInputs = [ pkgs.openssl pkgs.cacert ];

  postBuild = ''
    rm -rf $out/bin/hoc.d
  '';
}
