use core::panic;

use clap::Parser;

use mipstenite::{parser::mock_parser, virtual_machine::VirtualMachine, bytecode::{Bytecode, AsmInstruction}, debug_table::CompileDebugInfo, server::establish_connection, err_util::setup_logger};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
	#[clap(long, short, action)]
	cleanup: bool,

	#[clap(long, short)]
	debug: bool,
}

fn main() {

	setup_logger();

	let args = Args::parse();
	if args.cleanup {
			// remove all files with numbers in the name ending with .bin extension
			let files = std::fs::read_dir(".").unwrap();
			for file in files {
				let file = file.unwrap();
				let file_name = file.file_name();
				let file_name = file_name.to_str().unwrap();
				if file_name.ends_with(".bin") {
					let file_name = file_name.split(".bin").collect::<Vec<&str>>()[0];
					if file_name.parse::<u32>().is_ok() {
						std::fs::remove_file(file.path()).unwrap();
					}
				}
			}
		std::process::exit(0);
		}
	if args.debug {
		establish_connection();
	}
	panic!("panic");

	log::trace!("a trace example");
    log::debug!("deboogging");
    log::info!("such information");
    log::warn!("o_O");
    log::error!("boom");

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
		li $t2, -5
		add $t3, $t1, $t2"#;
		let result = mock_parser(lxr_src);
		let asm_instructions: Vec<AsmInstruction>;
		let mut byc_translations: Vec<Vec<Bytecode>> = Vec::new();

		if result.is_ok() {
			asm_instructions = result.unwrap().1;
		} else {
			println!("{:#?}", result);
			std::process::exit(1);
		}

		// construct a HashMap that maps line_number or usize to a tupe of (instruction, Vec<Bytecode>)
		// this allows for easy access of assembly instruction and generated bytecode for every line
		let compile_debug_info = CompileDebugInfo::new(asm_instructions.clone());

		let mut byc_instructions = asm_instructions.into_iter().map(|i| {
			let byc = i.to_bytecode();
			byc_translations.push(byc.clone());
			byc
		}).flatten().collect::<Vec<Bytecode>>();
		byc_instructions.push(Bytecode::TERMINATOR);

		let mut vm = VirtualMachine::new();
		vm.init(Default::default(), byc_instructions);
		vm.setup_debug(compile_debug_info);

		// Serialize the VM to a file
		vm.dump();

		// Deserialize the VM from a file
		let mut vm2: VirtualMachine = VirtualMachine::new().load("vm.bin");

		loop {
			match vm2.execute() {
				Ok(_) => {},
				Err(_) => break
			}
		}

		vm2.runtime_dbg.print_debug_info();

		println!("{:#?}", vm2);

}
