{
  description = "Hits-Of-Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-darwin"
        "aarch64-linux"
      ];
      forEachSupportedSystem = f:
        nixpkgs.lib.genAttrs supportedSystems (system:
          f {
            pkgs = import nixpkgs {
              inherit system;
            };
          });
    in {
      devShells = forEachSupportedSystem ({pkgs}: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ 
            pkg-config
            openssl
          ];
        };
      });
    };
}
