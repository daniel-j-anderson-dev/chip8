
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
       1. update vf
13. if first nibble is 4

    1. if vx == last 2 nibbles
       1. update vf
14. if first nibble is 5 and last nibble is 0

    1. if vx != vy
       1. update vf
15. if first nibble is 6

    1. vx = last 2 nibbles
16. if first nibble is 7

    1. vx += last 2 nibbles
17. if first nibble is 8

    1. if last nibble is 0
       1. vx = vy
    2. if last nibble is 1
       1. vx |= vy
    3. if last nibble is 2
       1. vx &= vy
    4. if last nibble is 3
       1. vx ^= vy
    5. if last nibble is 4
       1. vx += vy
       2. vf = 1 on carry
    6. if last nibble is 5
       1. vx -= vy
       2. vf = 0 on borrow
    7. if last nibble is 6
       1. vx >>= vy
       2. vf = old LSB
    8. if last nibble is 7
       1. vx =- vy
       2. vf = 0 on borrow
    9. if last nibble is E
       1. vx <<= vy
       2. vf = old MSB
18. if first nibble is 9

    1. if vx == vy
       1. update vf
19. ANNN 			i := NNN
20. if first nibble is A

    1. i = last 3 nibbles
21. if first nibble is B

    1. i = last 3 nibbles + v0
22. if first nibble is C

    1. vx = random_number & last 2 nibbles
23. if first nibble is D

    1. draw sprite vx vy n
    2. vf = 1 on collision
24. if vx -key
25. EX9E 			if vx -key then 	Is a key not pressed?
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
