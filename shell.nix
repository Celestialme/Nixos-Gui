{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    
    nativeBuildInputs = [ 
      pkgs.dbus
      pkgs.pkgconfig
      pkgs.openssl
      pkgs.sass
      pkgs.glib
      pkgs.cairo
      pkgs.pango
      pkgs.atk
      pkgs.gdk-pixbuf
      pkgs.libsoup
      pkgs.gtk3
      pkgs.webkitgtk
      pkgs.librsvg
      pkgs.patchelf

     ];
     buildInputs = [ 
      pkgs.nodejs

     ];
}