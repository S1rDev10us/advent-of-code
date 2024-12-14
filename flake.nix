{
  description = "A basic template for bevy projects";
  inputs = {
    nixpkgs.url = "nixpkgs/24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    allSystems = ["x86_64-linux"];
    overlays = [(import rust-overlay)];
    forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {pkgs = import nixpkgs {inherit system overlays;};});
  in {
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          pkg-config
          lld
          hyperfine
          # rust
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          # For commitlint
          nodejs_22
          # # Command runner
          # just
          samply
        ];
        shellHook = ''
          npm install
          npm run prepare
        '';
      };
    });
  };
}
