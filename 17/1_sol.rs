struct Machine<'a> {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: &'a [u8],
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(a: usize, b: usize, c: usize, program: &'a [u8]) -> Self {
        Self {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            program: program,
            pc: 0,
        }
    }

    fn combo_to_int(&self, arg: u8) -> Option<usize> {
        match arg {
            literal @ 0..=3 => Some(literal as usize),
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            _ => None,
        }
    }

    fn div(&mut self, arg: u8) -> usize {
        let numerator = self.reg_a;
        let denominator = 2usize.pow(self.combo_to_int(arg).unwrap() as u32);
        numerator / denominator
    }

    fn adv(&mut self, arg: u8) {
        self.reg_a = self.div(arg);
        self.pc += 2;
    }

    fn bxl(&mut self, arg: u8) {
        self.reg_b = self.reg_b ^ (arg as usize);
        self.pc += 2;
    }

    fn bst(&mut self, arg: u8) {
        self.reg_b = self.combo_to_int(arg).unwrap() % 8;
        self.pc += 2;
    }

    fn jnz(&mut self, arg: u8) {
        if self.reg_a == 0 {
            self.pc += 2;
        } else {
            self.pc = arg as usize;
        }
    }

    fn bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.pc += 2;
    }

    fn out(&mut self, arg: u8){
        self.pc += 2;
        print!("{},", (self.combo_to_int(arg).unwrap() % 8) as u8)
    }

    fn bdv(&mut self, arg: u8) {
        self.reg_b = self.div(arg);
        self.pc += 2;
    }

    fn cdv(&mut self, arg: u8) {
        self.reg_c = self.div(arg);
        self.pc += 2;
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                0 => self.adv(self.program[self.pc + 1]),
                1 => self.bxl(self.program[self.pc + 1]),
                2 => self.bst(self.program[self.pc + 1]),
                3 => self.jnz(self.program[self.pc + 1]),
                4 => self.bxc(),
                5 => self.out(self.program[self.pc + 1]),
                6 => self.bdv(self.program[self.pc + 1]),
                7 => self.cdv(self.program[self.pc + 1]),
                _ => panic!(),
            };
        }
    }
}

fn main() {
    let input = [2,4,1,5,7,5,1,6,0,3,4,0,5,5,3,0];
    let a = 105843716614554;
    let mut machine = Machine::new(a, 0, 0, &input[..]);
    machine.run();
    println!();
}
