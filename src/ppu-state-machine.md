Reference: https://gbdev.io/pandocs/Rendering.html 

# PPU State Machine

## Mode 2: OAM Scan
The PPU searches the OAM (Object Attribute Memory) for objects in this mode.. This mode takes exactly 80 dots (T-cycles) to complete.

CPU Access: The only areas of the video memory accessible during this time are the VRAM and the CGB palettes (unnecessary for now). If the CPU tries to read OAM it receives 0xFF and writes are ignored.

## Mode 3: Pixel Transfer
The PPU typically outputs one pixel to the screen per dots, resulting in 160 dots + 12 additional dots for tile fetching, resulting in at least 172 dots spent in Mode 3. It is possible to exceed 172 dots due to certain features that cause the rendering process to stall. Any dots over 172 are taken directly from the dot budget for Mode 0, upto a total of 289 dots. The number of dots above 172 are determined as follows:


## Mode 0:

## Mode 1: 
