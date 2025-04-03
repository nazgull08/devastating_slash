{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    xorg.libX11
    xorg.libXi
    xorg.libXcursor
    xorg.libXrandr
    libxkbcommon  

    mesa
    mesa.drivers
    libGL

    libGLU            
    glew              
    vulkan-loader     
    vulkan-tools      
    glxinfo          
  ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  shellHook = ''
    echo "devastating slash env loaded."
    export RUST_LOG=info
    export LD_LIBRARY_PATH="${pkgs.libGL}/lib:${pkgs.libGLU}/lib:${pkgs.glew}/lib:${pkgs.mesa}/lib:${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"
  echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
    echo "RUST_LOG=$RUST_LOG"
  '';
}
