# MARIE compiler and interpreter

To learn all about MARIE: <a href="http://www.cs.uni.edu/~fienup/cs041s11/lectures/Supplement_MARE_AL.pdf"> read more... </a>

### Usage

`cargo run [FILENAME]`

Example `program to square a number`:

```
Input
Store X
Store Y
Store G

Loop, Load Y / load y
Subt Z /accum = y - 1
Store Y
Skipcond 400
Jump Square
Jump End

Square, Load X
Add G
Store X
Jump Loop

End, Load X
Output
Halt

X, Dec 0
Y, Dec 0
G, Dec 0
Z, Dec 1
```

To run other examples, visit the `examples\working` directory !!! </br>

TODO:

1. Add support for `JUMP`, `SkipCond` and functions (Done)
2. Add support for `JNS`, `LOADI`, `ADDI`
