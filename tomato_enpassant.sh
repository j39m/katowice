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
  xrandr --output HDMI1 --auto --left-of LVDS1; 
  pactl set-card-profile 0 output:hdmi-stereo+input:analog-stereo; 
else 
  printf "Doing stuff for HDMI disconnection.\n"; 
  xrandr --auto; 
  pactl set-card-profile 0 output:analog-stereo+input:analog-stereo; 
fi; 

sh ~/.fehbg; 
