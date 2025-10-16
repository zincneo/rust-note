{
  description = "egui-examples development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            xorg.libX11
            xorg.libXi
            xorg.libXcursor
            libGL
            libxkbcommon
          ];

          shellHook = ''
            echo "🎨 egui开发环境已启动！"
            unset WAYLAND_DISPLAY
            export LD_LIBRARY_PATH="${pkgs.xorg.libX11}/lib:${pkgs.libGL}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXi}/lib:${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
