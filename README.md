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


# Installation: create default.nix from code below and run nix-build.

nix-build --expr 'with import \<nixpkgs> {}; callPackage ./default.nix {}'

default.nix
  
  🠋🠋🠋
```
{    pkgs ? import <nixpkgs> {} }:
let
  version = "0.1.0";
  
in pkgs.stdenv.mkDerivation {
  name = "Nixos_Gui-${version}";
  system = "x86_64-linux";
  src = pkgs.fetchurl {
        url = "https://github.com/celestialme/Nixos-Gui/archive/47ee6b29713ba04e4a088b31778fd1e0e358f087.tar.gz";
        sha256= "sha256-x4ld1SIg7q8qBYifgY89mXY3vl2S0JX5nsKsaEzUo6Y=";
      };
  nativeBuildInputs = [
    pkgs.autoPatchelfHook
  ];
  buildInputs = [
       pkgs.openssl
       pkgs.webkitgtk
  ];

  unpackPhase = "
  tar -xvf $src
  ";
  installPhase = ''
     mkdir -p $out
     echo "current dir"
     ls -a
     
    cp -r ./Nixos-*/usr/* $out
  '';
}



```
# Tinkering: create local dev env
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

# OR fork and build with actions yourself
![Alt text](screenshots/1.png?raw=true "Optional Title")
![Alt text](screenshots/3.png?raw=true "Optional Title")
![Alt text](screenshots/4.png?raw=true "Optional Title")
![Alt text](screenshots/5.png?raw=true "Optional Title")
![Alt text](screenshots/8.png?raw=true "Optional Title")
![Alt text](screenshots/6.png?raw=true "Optional Title")
![Alt text](screenshots/7.png?raw=true "Optional Title")
