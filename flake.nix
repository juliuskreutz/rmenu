{
  description = "Rwm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    {
      overlays.default = _: prev: {
        rmenu = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };
      overlays.rmenu = self.overlays.default;
    }
    // (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = crane.mkLib pkgs;

      commonArgs = {
        # src = craneLib.cleanCargoSource (craneLib.path ./.);

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type: (builtins.baseNameOf path) == "rmenu_run" || (craneLib.filterCargoSources path type);
        };

        inherit (craneLib.crateNameFromCargoToml {cargoToml = ./rmenu/Cargo.toml;}) pname version;

        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          libxkbcommon
          glib
          pango
        ];
      };

      rmenu = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          postInstall = ''
            cp rmenu_run $out/bin/
          '';
        });
    in {
      checks = {
        inherit rmenu;
      };

      formatter = pkgs.alejandra;

      packages.default = rmenu;
      packages.rmenu = rmenu;

      apps.default = flake-utils.lib.mkApp {
        drv = rmenu;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
      };
    }));
}
