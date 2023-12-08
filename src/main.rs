use mipstenite::{parser::mock_parser, virtual_machine::VirtualMachine, bytecode::{Bytecode, AsmInstruction}, panic_hook::set_panic_hook};

fn main() {
    let src = r#"
        # ------------------------------------------------------------------
        	
        	.text
        main:
        	lw	$s0, x		# Reg $s0 = x
        	lw	$s1, y		# Reg $s1 = y
        
        	# Call function
        	move	$a0, $s0	# Argument 1: x ($s0)
        	jal	fun		# Save current PC in $ra, and jump to fun
        	move	$s1,$v0		# Return value saved in $v0. This is y ($s1)
        
        	# Print msg1
        	li	$v0, 4		# print_string syscall code = 4
        	la	$a0, msg1
        	syscall
        
        fun:	# This function overwrites $s0 and $s1
        	# We should save those on the stack
        	# This is PUSH'ing onto the stack
        	addi $sp,$sp,-4		# Adjust stack pointer
        	sw $s0,0($sp)		# Save $s0
        	addi $sp,$sp,-4		# Adjust stack pointer
        	sw $s1,0($sp)		# Save $s1
        
        	# Save the return value in $v0
        	move $v0,$s1
        
        	.data
        x:	.word 5
        y:	.word 0
        msg1:	.asciiz	"y="
        lf:     .asciiz	"\n"
        "#;

        let lxr_src = r#"li $t1, 45
		li $t2, 5
		add $t3, $t1, $t2"#;
		let result = mock_parser(lxr_src);
		let asm_instructions: Vec<AsmInstruction>;
		let mut byc_translations: Vec<Vec<Bytecode>> = Vec::new();
		if result.is_ok() {
			asm_instructions = result.unwrap().1;
			for i in asm_instructions {
				byc_translations.push(i.to_bytecode());
			}
		}

		let byc_instructions = byc_translations.into_iter().flatten().collect::<Vec<Bytecode>>();

		let mut vm = VirtualMachine::new();
	
		vm.init(Default::default(), byc_instructions);

		println!("{:#?}", vm);


		// DEBUG: remove later
		for i in 0..9 {
			println!("Step {}", i);
			vm.execute();
		}

		println!("{:#?}", vm);

}
