# MARIE compiler in rust

To learn all about MARIE: <a href="http://www.cs.uni.edu/~fienup/cs041s11/lectures/Supplement_MARE_AL.pdf"> read more... </a>

### Usage

`cargo run [FILENAME]`

Example `[FILENAME]`:

```
Input      /Takes user input
Store X    /Stores user input to X
Input      /Takes user input
Store Y    /Stores user input to Y
Load X     /Load X to AC
Add Y     /Add Y to X
Store Z    /Store addition to Z
Output     /Print Z
HALT       /End program

/Variable decleration
Z, HEX 0x04
X, HEX 0
Y, HEX 0

```

TODO:

1. Add support for `JNS`, `JUMP` and functions
