#! /bin/bash

# tomato_enpassant is my personal script for automatically
# doing everything to set-up displays properly when run.

# stopgap for feh using a relative path for its wallpaper.
# The below should just reflect whatever directory makes
# the wallpaper path coherent. 
cd ~; 

xrandr_output=$(xrandr -q); 

killall compton; 

if [[ "$xrandr_output" =~ "HDMI1 connected" ]]; then 
  printf "Doing stuff for HDMI connection.\n"; 
  xrandr --output HDMI1 --auto --left-of LVDS1; 
else 
  printf "Doing stuff for HDMI disconnection.\n"; 
  xrandr --auto; 
fi; 

{ compton > /dev/null 2>&1 & }; 
sh ~/.fehbg; 
killall -s SIGUSR1 tint2; 
