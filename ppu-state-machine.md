Reference: https://gbdev.io/pandocs/Rendering.html 

# PPU State Machine

## Mode 2: OAM Scan
The PPU searches the OAM (Object Attribute Memory) for objects in this mode.. This mode takes exactly 80 dots (T-cycles) to complete.

**CPU Access**: The only areas of the video memory accessible during this time are the VRAM and the CGB palettes (unnecessary for now). If the CPU tries to read OAM it receives 0xFF and writes are ignored.

## Mode 3: Pixel Transfer
The PPU typically outputs one pixel to the screen per dots, resulting in 160 dots + 12 additional dots for tile fetching, resulting in at least 172 dots spent in Mode 3. It is possible to exceed 172 dots due to certain features that cause the rendering process to stall. Any dots over 172 are taken directly from the dot budget for Mode 0, upto a total of 289 dots. The number of dots above 172 are determined as follows:

Background Scrolling: The PPU stalls for SCX % 8 dots where SCX is the scroll register
Window: A flat 6 dot penalty is incurred when window drawing is enabled.
Objects: A flat 6 dot penalty plus between 0-5 extra dots determined as follows: (let P denote the left-most pixel)
- Check whether the P is on a background or a window tile
- If that tile has not yet been considered by a previous obj, incur a penalty of max(0, N - 2) where N is the number of pixels of that tile to the right of P.
  *Exception*: If the object's OAM X position is 0, incur 5 additional dots anyway.

**CPU Access**: the CPU is forbidden from accessing both OAM and VRAM in this mode

## Mode 0: Horizontal Blank
The PPU waits until a full 456 dots have been completed (starting at 0 dots before Mode 2). The total number of dots spent in this mode is dependent on the total number of dots used by Mode 3. Since Mode 2 is 80 dots, Mode 0's length is determined by (456 - 80) - Mode 3's length = 376 - Mode 3's length.

**CPU Access**: The CPU can access both OAM and VRAM during this mode.

*Transition Remark*: this is the first branching transition point. If (144 * 456) dots have passed from the start of Mode 2 (143 scanlines have been processed fully) we transition to Mode 1. Otherwise, we transition back to Mode 2.

## Mode 1: 
The VBLANK interrupt is triggered, causing the VBLANK interrupt handler to run. The PPU does nothing for 4560 dots and then transitions back to Mode 2.

**CPU Access**: The CPU can access both OAM and VRAM during this mode.
