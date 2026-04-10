{ pkgs, ... }:

{
  packages = with pkgs; [
    pkg-config
    glib
    at-spi2-atk
    libsoup_3
    webkitgtk_4_1
    xdotool
    gsettings-desktop-schemas
  ];

  android.enable = true;

  enterShell = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS"
    export RUSTC_WRAPPER=""
  '';
}
