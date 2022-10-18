(*
  Terms:
  - memory: the Intcode program represented as a list of integers
  - address: a position in memory
  - instruction: an "opcode" such as 1, 2, 99 etc.
  - instruction parameters:
    the values immediately after an opcode. for example, in the
    instruction [1, 2, 3, 4] 1 is the opcode, and
    2, 3, 4 are the instruction parameters. 99 contains just an
    opcode and *no* parameters.
  - instruction pointer:
    the address of the current instruction. this is initially 0.
    after an instruction finishes, the instruction pointer increases
    by the number of values in the instruction. for example, for
    the instruction [1, 2, 3, 4] => 4 (1 opcode + 3 parameters)

  General approach:
  1. initialize memory (in some way, typically setting the value at address 1 and 2?)
  2. initialize address pointer to 0  
  3. parse instruction (fallible)
    - add
    - multiply
    - other?
    - halt => Ok ()
    - error => Error
  4. perform operation
    - mutate memory (fallible)
    - update instruction pointer
  5. repeat
 *)
