1. if first nibble is 0
   1. if last 3 nibbles are 0E0
      1. clear
   2. if last 3 nibbles are 0EE
      1. return
         1. pop return address from call stack
         2. jump to above address
2. if first nibble is 1
   1. jump
      1. pc = last 3 nibbles
3. if first nibble is 2
   1. call subroutine
      1. push current address on call stack
      2. pc = last 3 nibbles
4. if first nibble is 3
   1. if vx != last 2 nibbles
      1. update vf
5. if first nibble is 4
   1. if vx == last 2 nibbles
      1. update vf
6. if first nibble is 5 and last nibble is 0
   1. if vx != vy
      1. update vf
7. if first nibble is 6
   1. vx = last 2 nibbles
8. if first nibble is 7
   1. vx += last 2 nibbles
9. if first nibble is 8
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
10. if first nibble is 9
    1. if vx == vy
    2. update vf
11. if first nibble is A
    1. i = last 3 nibbles
12. if first nibble is B
    1. i = last 3 nibbles + v0
13. if first nibble is C
    1. vx = random_number & last 2 nibbles
14. if first nibble is D
    1. draw sprite vx vy n
    2. vf = 1 on collision
15. if first nibble is E
    1. if vx -key
       1. update vf
       2. check key not pressed
    2. if vx key
       1. update vf
       2. check key pressed
16. if first nibble is F
    1. if last 2 nibbles are 07
       1. vx = delay
    2. if last 2 nibbles are 0A
       1. vx = key
       2. wait for a keypress
    3. if last 2 nibbles are 15
       1. delay = vx
    4. if last 2 nibbles are 18
       1. buzzer = vx
    5. if last 2 nibbles are 1E
       1. i += vx
    6. if last 2 nibbles are 29
       1. i = hex vx
       2. set i to a hex character
    7. if last 2 nibbles are 33
       1. decode vx into binary-coded decimal
    8. if last 2 nibbles are 55
       1. save v0 - vx to i - (i + x)
    9. if last 2 nibbles are 65
       1. load v0 - vx from i - (i + x)