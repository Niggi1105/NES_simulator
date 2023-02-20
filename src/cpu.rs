pub struct CPU{
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    status_reg: u8,
    program_counter: u16,

}
impl CPU{
    pub fn new() -> Self{
        CPU{
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status_reg: 0,
            program_counter: 0,
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;

        loop {
            let opc = program[self.program_counter as usize];
            self.program_counter += 1;
            match opc{
                //BRK
                0x00 => {
                    return
                }
                //LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter +=1;
                    self.reg_a = param;
    
                    if self.reg_a == 0 {
                        self.status_reg = self.status_reg | 0b0000_0010;
                    } else {
                        self.status_reg = self.status_reg & 0b1111_1101;
                    }
    
                    if self.reg_a & 0b1000_0000 != 0 {
                        self.status_reg = self.status_reg | 0b1000_0000;
                    } else {
                        self.status_reg = self.status_reg & 0b0111_1111;
                    }
                }
                //LAX
                0xAA => {
                    self.reg_x = self.reg_a;
                    if self.reg_a == 0 {
                        self.status_reg = self.status_reg | 0b0000_0010;
                    } else {
                        self.status_reg = self.status_reg & 0b1111_1101;
                    }
    
                    if self.reg_a & 0b1000_0000 != 0 {
                        self.status_reg = self.status_reg | 0b1000_0000;
                    } else {
                        self.status_reg = self.status_reg & 0b0111_1111;
                    }
                }
                _ => todo!()
            }
        }
    }
}


#[cfg(test)]
mod test {
   use super::*;
 
   #[test]
   fn test_0xa9_lda_immidiate_load_data() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.reg_a, 0x05);
       assert!(cpu.status_reg & 0b0000_0010 == 0b00);
       assert!(cpu.status_reg & 0b1000_0000 == 0);
   }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status_reg & 0b0000_0010 == 0b10);
    }
}