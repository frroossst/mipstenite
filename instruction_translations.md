# Load Instructions

| Instruction | Translation |
|-------------|-------------|
| li $reg imm | PUSH imm; SET $reg |


# Arithmetic Instructions
| Instruction | Translation |
|-------------|-------------|
| add $c, $a, $b | GETP $a; GETP $b; ADD; SETO $c |

