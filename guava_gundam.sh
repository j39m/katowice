#! /bin/bash

# guava_gundam: ghetto screen brightness changer. 
# usage: [bash] guava_gundam.sh [increase/decrease] [change amount]
# defaults to increasing brightness by 65. 

brightness_file="/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-LVDS-1/intel_backlight/brightness"; 
max_brightness_file="/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-LVDS-1/intel_backlight/max_brightness"; 
brightness_direction=${1-increase} ; 
brightness_increment=${2-52} ;
brightness_current=$(cat "$brightness_file"); 
brightness_max=$(cat "$max_brightness_file"); 

# determine new value 
if [ "$brightness_direction" = "increase" ]; then 
  brightness_new=$(( $brightness_current+$brightness_increment )); 
else 
  brightness_new=$(( $brightness_current-$brightness_increment )); 
fi 

# apply sanity check on new value 
while [ $brightness_new -lt 0 ]; do 
  ((brightness_new++)); 
done 
while [ $brightness_new -gt "$brightness_max" ]; do 
  ((brightness_new--)); 
done 

# write new value 
echo "$brightness_new" > "$brightness_file"; 

exit 0; 
