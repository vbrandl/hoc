{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { }
, callPackage ? pkgs.callPackage
}:

let
  cargoNix = callPackage ./Cargo.nix {
    # defaultCrateOverrides = pkgs.defaultCrateOverrides // {
    #   libgit2-sys = attrs: {
    #     buildInputs = [ pkgs.openssl ];
    #   };
    # };
  };

  hoc = cargoNix.rootCrate.build;
in
pkgs.symlinkJoin {
  name = hoc.name;
  version = hoc.crateVersion;
  paths = [ hoc ];

  # buildInputs = [ pkgs.openssl pkgs.cacert ];

  postBuild = ''
    rm -rf $out/bin/hoc.d
  '';
}
