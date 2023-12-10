{
  description = "A Rust port of node-bunyan";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { crane, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.lib.${system};

        # By default, Crane will only pick up rust-relevant files.
        # This makes it also include the `.log` files needed for tests.
        logFileFilter = path: _type: builtins.match ".*log$" path != null;
        cargoOrLogFilter = path: type:
          (logFileFilter path type) || (craneLib.filterCargoSources path type);
      in
      {
        packages.default = craneLib.buildPackage
          {
            src = pkgs.lib.cleanSourceWith {
              src = craneLib.path ./.;
              filter = cargoOrLogFilter;
            };

            nativeBuildInputs = [ pkgs.libiconv ];
          };
      });
}
