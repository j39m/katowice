#! /bin/bash

# tomato_enpassant is my personal script for automatically
# doing everything to set-up displays properly when run.

# stopgap for feh using a relative path for its wallpaper.
# The below should just reflect whatever directory makes
# the wallpaper path coherent. 
cd ~; 

xrandr_output=$(xrandr -q); 

if [[ "$xrandr_output" =~ "HDMI1 connected" ]]; then 
  printf "Doing stuff for HDMI connection.\n"; 
  xrandr --output HDMI1 --auto --left-of eDP1; 
  pactl set-card-profile alsa_card.pci-0000_00_03.0 output:hdmi-stereo ; 
else 
  printf "Doing stuff for HDMI disconnection.\n"; 
  xrandr --auto; 
  pactl set-card-profile alsa_card.pci-0000_00_1b.0 output:analog-stereo ;
fi; 

sh ~/.fehbg; 
