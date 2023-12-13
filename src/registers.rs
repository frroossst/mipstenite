pub struct Register {
    pub name: String,
    pub value: u32,
}

impl Register {
    pub fn new(name: String) -> Register {
        if !name.starts_with("$") {
            panic!("Register name must start with $");
        }
        Register {
            name,
            value: 0,
        }
    }

}

pub fn register_to_addr(reg: String) -> Option<u32> {
    let reg_name = reg.as_str();
    match reg_name {
        "$zero" => { Some(0) }
        "$at" => { Some(1) }
        "$v0" => { Some(2) }
        "$v1" => { Some(3) }
        "$a0" => { Some(4) }
        "$a1" => { Some(5) }
        "$a2" => { Some(6) }
        "$a3" => { Some(7) }
        "$t0" => { Some(8) }
        "$t1" => { Some(9) }
        "$t2" => { Some(10) }
        "$t3" => { Some(11) }
        "$t4" => { Some(12) }
        "$t5" => { Some(13) }
        "$t6" => { Some(14) }
        "$t7" => { Some(15) }
        "$s0" => { Some(16) }
        "$s1" => { Some(17) }
        "$s2" => { Some(18) }
        "$s3" => { Some(19) }
        "$s4" => { Some(20) }
        "$s5" => { Some(21) }
        "$s6" => { Some(22) }
        "$s7" => { Some(23) }
        "$t8" => { Some(24) }
        "$t9" => { Some(25) }
        "$k0" => { Some(26) }
        "$k1" => { Some(27) }
        "$gp" => { Some(28) }
        "$sp" => { Some(29) }
        "$fp" => { Some(30) }
        "$ra" => { Some(31) }
        "$hi" => { Some(32) }
        "$lo" => { Some(33) }
        _ => { None }
    }
}

pub fn addr_to_register(addr: u32) -> Option<Register> {
    match addr {
        0 => { Some(Register::new("$zero".to_string())) }
        1 => { Some(Register::new("$at".to_string())) }
        2 => { Some(Register::new("$v0".to_string())) }
        3 => { Some(Register::new("$v1".to_string())) }
        4 => { Some(Register::new("$a0".to_string())) }
        5 => { Some(Register::new("$a1".to_string())) }
        6 => { Some(Register::new("$a2".to_string())) }
        7 => { Some(Register::new("$a3".to_string())) }
        8 => { Some(Register::new("$t0".to_string())) }
        9 => { Some(Register::new("$t1".to_string())) }
        10 => { Some(Register::new("$t2".to_string())) }
        11 => { Some(Register::new("$t3".to_string())) }
        12 => { Some(Register::new("$t4".to_string())) }
        13 => { Some(Register::new("$t5".to_string())) }
        14 => { Some(Register::new("$t6".to_string())) }
        15 => { Some(Register::new("$t7".to_string())) }
        16 => { Some(Register::new("$s0".to_string())) }
        17 => { Some(Register::new("$s1".to_string())) }
        18 => { Some(Register::new("$s2".to_string())) }
        19 => { Some(Register::new("$s3".to_string())) }
        20 => { Some(Register::new("$s4".to_string())) }
        21 => { Some(Register::new("$s5".to_string())) }
        22 => { Some(Register::new("$s6".to_string())) }
        23 => { Some(Register::new("$s7".to_string())) }
        24 => { Some(Register::new("$t8".to_string())) }
        25 => { Some(Register::new("$t9".to_string())) }
        26 => { Some(Register::new("$k0".to_string())) }
        27 => { Some(Register::new("$k1".to_string())) }
        28 => { Some(Register::new("$gp".to_string())) }
        29 => { Some(Register::new("$sp".to_string())) }
        30 => { Some(Register::new("$fp".to_string())) }
        31 => { Some(Register::new("$ra".to_string())) }
        32 => { Some(Register::new("$hi".to_string())) }
        33 => { Some(Register::new("$lo".to_string())) }
        _ => { None }
    }
}

pub struct PrettyFmtRegister<'a>(pub &'a [u32; 32]);

impl<'a> std::fmt::Debug for PrettyFmtRegister<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The registers should be formatted as `name: value`
        f.debug_list()
        .entries(self.0.iter().enumerate().map(|(index, &value)| {
            if let Some(register_info) = addr_to_register(index as u32) {
                format!("{}: {}", register_info.name, value)
            } else {
                format!("UnknownRegister: {}", value)
            }
        }))
        .finish()
    }
}
