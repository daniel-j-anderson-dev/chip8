
CHIP-8 Instruction Set

1. N is a number between 0 and 15.
2. NN is a number between 0 and 255.
3. NNN is an address between 0 and 4095.
4. vx and vy are registers (0-F).
5. i is the memory index register.
6. Instructions in gray rows may modify the vF register.
7. Instruction switches
8. clear
   1. if 00E0
9. return
   1. if00EE
      1. pop return address from call stack
      2. jump to above address
10. jump
    1. if first nibble is 1
       1. jump to address above
11. call subroutine
    1. if first nibble is 2
       1. save address on call stack
       2. jump to address defined by last 3 nibbles
12. if first nibble is 3
    1. if vx != last 2 nibbles
       1. store flag value in vf
13. if first nibble is 4
    1. if vx == last 2 nibbles
       1. store flag value in vf
14. if first nibble is 5 and last nibble is 0
    1. if vx != vy
       1. store flage value in vf
15. if first nibble is 6
    1. assign value of vx to last 2 nibbles
16. 6XNN			vx := NN
    7XNN 			vx += NN
    8XY0 			vx := vy
    8XY1 			vx |= vy 			Bitwise OR
    8XY2 			vx &= vy 			Bitwise AND
    8XY3 			vx ^= vy 			Bitwise XOR
17. 8XY4 			vx += vy 			vf = 1 on carry
    8XY5 			vx -= vy 			vf = 0 on borrow
    8XY6 			vx >>= vy 		vf = old least significant bit
    8XY7 			vx =- vy 			vf = 0 on borrow
    8XYE 			vx <<= vy 		vf = old most significant bit
18. 9XY0 			if vx == vy then
    ANNN 			i := NNN
    BNNN 			jump0 NNN 		Jump to address NNN + v0
    CXNN 			vx := random NN 	Random number 0-255 AND NN
19. DXYN 			sprite vx vy N 		vf = 1 on collision
20. EX9E 			if vx -key then 	Is a key not pressed?
    EXA1 			if vx key then 		Is a key pressed?
    FX07 			vx := delay
    FX0A 			vx := key 			Wait for a keypress
    FX15 			delay := vx
    FX18 			buzzer := vx
    FX1E 			i += vx
    FX29 			i := hex vx 		Set i to a hex character
    FX33 			bcd vx 			Decode vx into binary-coded decimal
    FX55 			save vx		 	Save v0-vx to i through (i+x)
    FX65 			load vx 			Load v0-vx from i through (i+x)

Variables:

    register 0-F

    i -> memory index register

    memory

delay timer register

sound timer register

call stack
