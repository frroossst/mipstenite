# Load Instructions

| Instruction | Translation |
|-------------|-------------|
| li $reg imm | PUSH imm; SET $reg |


# Arithmetic Instructions
| Instruction | Translation |
|-------------|-------------|
| add $c, $a, $b | GETP $a; GETP $b; ADD; SETO $c |
| addi $c, $a, imm | PUSH imm, 


# Virtual Machine Instructions
| Translation | Description |
|-------------|-------------|
| ALLOC | Allocates memory on the heap; returns a void pointer |
| DEBUG | Returns current state of the VM for debugging |
| EXIT | Exits the VM, 
| HALT | Stops the VM indefinitely until a signal is received |
| STDIN | Takes an input from the console |
| STDOUT | Prints to the console |
| STRACE | Adds current instruction to the stack trace; usually before before execution begins |

