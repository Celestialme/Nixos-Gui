# about

Gui application to manage NIXOS build.
# usage
### packages section 
you can search for packages. you can mark them (they will be added to configuration.nix) or you can download directly (nix-env -iA will be used)
### options section  
you can manage your options from there. changed will update configuration.nix accordingly. Application will ask for manual revision of configuration.nix before update.
### system section   
you can controll channels, see generations, update channels, rebuild.

downloads and rebuild has progress bar indicator.


# create local dev env
``` 
git clone https://github.com/Celestialme/Nixos-Gui.git 
cd Nixos-Gui
nix-shell


npm install

npm run dev
npm run tauri dev

```
# OR download release 
https://github.com/Celestialme/Nixos-Gui/releases
# OR fork and build with actions yourself
