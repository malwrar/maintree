#blackpill

Firmware for various projects targeting the [blackpill][1] dev board.

## How to flash

1. *Put the board in DFU mode*: Locate the NRST & BOOT0 buttons, which should
   be right next to each other.  Press both buttons at the same time (I use my
   pointer finger to press both) and hold them for a second or so. Then,
   release the NRST button while still holding the BOOT0 button, then after a
   second or so release the BOOT0 button.
2. *Build and flash the firmware*: Use the `./build_and_flash.sh` script to do
   this, simply provide the name of a target in `./src/bin/`. Some of the
   commands in that script require some additional system tools, on debian you
   can get them with:

   ```
   sudo apt install -y dfu-util gcc-arm-none-eabi
   ```
3. *Reset the board*: Press the NRST button to exit DFU mode and boot into the
   newly flashed firmware.

[1]: git@github.com:malwrar/maintree.git
