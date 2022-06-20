# about

Gui application to manage NIXOS build.
# usage
### packages section 
you can search for packages. you can mark them (they will be added to configuration.nix using rnix ast) or you can download directly (nix-env -iA will be used)
### options section  
you can manage your options from there. changed will update configuration.nix accordingly. Application will ask for manual revision of configuration.nix before update.
### system section   
you can controll channels, see generations, update channels, rebuild.

on the first run you have to let app create packages and options database as root which will be located at /etc/NIX_GUI. to modify configuration.nix or rebuild system you need to run nix-gui as root or with sudo. Installed packages and channels depend on user by which app is run.

downloads and rebuild has progress bar indicator.


# create local dev env
``` 
git clone https://github.com/Celestialme/Nixos-Gui.git 
cd Nixos-Gui
```
### open 2 terminals here:

terminal #1 run:
```
nix-shell
npm install
npm run dev
```
terminal #2 run:
```
nix-shell
npm run tauri dev
```

# OR download release and install
https://github.com/Celestialme/Nixos-Gui/releases

nix-build --expr 'with import \<nixpkgs> {}; callPackage ./default.nix {}'

default.nix
  
  🠋🠋🠋
```
{ stdenv, dpkg,  autoPatchelfHook,pkgs ? import <nixpkgs> {} }:
let
  version = "0.1.0";
  src = ./nixos-gui_0.1.0_amd64.deb;
in stdenv.mkDerivation {
  name = "Nixos_Gui-${version}";
  system = "x86_64-linux";
  inherit src;
  nativeBuildInputs = [
    autoPatchelfHook
    dpkg
  ];
  buildInputs = [
       pkgs.openssl
       pkgs.webkitgtk
  ];

  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    dpkg -x $src $out
    mv $out/usr/* $out
    rm -r $out/usr
  '';
}

```
# OR fork and build with actions yourself
