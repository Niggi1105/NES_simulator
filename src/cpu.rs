pub struct CPU{
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    status_reg: u8,
    program_counter: u16,
    memory: [u8;0xFFFF]
}

#[derive(Debug)]
pub enum AddressingMode{
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing
}

impl CPU{
    pub fn new() -> Self{
        CPU{
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status_reg: 0,
            program_counter: 0,
            memory: [0;0xFFFF]
        }
    }

    fn get_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode{
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.read_mem(self.program_counter) as u16,
            AddressingMode::ZeroPageX => self.read_mem(self.program_counter).wrapping_add(self.reg_x) as u16,
            AddressingMode::ZeroPageY => self.read_mem(self.program_counter).wrapping_add(self.reg_y) as u16,
            AddressingMode::Absolute => self.read_mem_u16(self.program_counter),
            AddressingMode::AbsoluteX => self.read_mem_u16(self.program_counter).wrapping_add(self.reg_x as u16),
            AddressingMode::AbsoluteY => self.read_mem_u16(self.program_counter).wrapping_add(self.reg_y as u16),
            // adds x register to zero page address and uses the resulting value as pointer to the address
            AddressingMode::IndirectX => {
                let ptr= self.read_mem(self.program_counter).wrapping_add(self.reg_x);
                let lo = self.read_mem(ptr as u16) as u16;
                let hi = self.read_mem(ptr.wrapping_add(1) as u16) as u16;
                (hi << 8) | (lo)
            }
            // uses zero page address to access address and adds the y register to the result
            AddressingMode::IndirectY =>{
                let base = self.read_mem(self.program_counter) as u16;
                let lo = self.read_mem(base) as u16;
                let hi = self.read_mem(base.wrapping_add(1)) as u16;
                ((hi << 8) | lo).wrapping_add(self.reg_y as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    
    fn update_z_and_c_flag(&mut self,val:u8) {
        if val == 0 {
            self.status_reg = self.status_reg | 0b0000_0010;
        } else {
            self.status_reg = self.status_reg & 0b1111_1101;
        }

        if val & 0b1000_0000 != 0 {
            self.status_reg = self.status_reg | 0b1000_0000;
        } else {
            self.status_reg = self.status_reg & 0b0111_1111;
        }
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_address(&mode);
        self.reg_a = self.read_mem(addr);
        self.update_z_and_c_flag(self.reg_a);
    }

    fn ldx(&mut self, mode: AddressingMode){
        let addr = self.get_address(&mode);
        self.reg_x = self.read_mem(addr);
        self.update_z_and_c_flag(self.reg_x);
    }

    fn ldy(&mut self, mode: AddressingMode){
        let addr = self.get_address(&mode);
        self.reg_y = self.read_mem(addr);
        self.update_z_and_c_flag(self.reg_y);
    }

    fn tax(&mut self){
        self.reg_x = self.reg_a;
        self.update_z_and_c_flag(self.reg_x)
    }

    fn inx(&mut self){
        if self.reg_x == 0xFF{
            self.reg_x = 0;
        }else{
            self.reg_x += 1;
        }
        println!("{:?}",self.reg_x);
        self.update_z_and_c_flag(self.reg_x)
    }
    
    fn iny(&mut self){
        if self.reg_y == 0xFF{
            self.reg_y = 0;
        }else{
            self.reg_y += 1;
        }
        self.update_z_and_c_flag(self.reg_y)
    }

    fn sta(&mut self, mode: AddressingMode){
        self.memory[self.get_address(&mode) as usize] = self.reg_a;
    }

    fn stx(&mut self, mode: AddressingMode){
        self.memory[self.get_address(&mode) as usize] = self.reg_x;
    }

    fn sty(&mut self, mode: AddressingMode){
        self.memory[self.get_address(&mode) as usize] = self.reg_y;
    }

    fn read_mem(&mut self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    fn read_mem_u16(&mut self, addr: u16) -> u16{
        let lo = self.read_mem(addr) as u16;
        let hi = self.read_mem(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn write_mem(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }

    fn write_mem_u16(&mut self, addr: u16, data: u16){
        let lo = data & 0x00FF;
        let hi = (data & 0xFF00) >> 8;
        self.write_mem(addr, lo as u8);
        self.write_mem(addr + 1, hi as u8);
    }

    //whipes all registers and sets program counter to addr stored at 0xFFFC
    pub fn reset(&mut self){
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.status_reg = 0;
        self.program_counter = self.read_mem_u16(0xFFFC);
    }

    //load progarm to memory an stores starting address
    pub fn load(&mut self, program: Vec<u8>){
        self.memory[0x8000..(0x8000+program.len())].copy_from_slice(&program);
        self.write_mem_u16(0xFFFC,0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self){
        loop {
            let opc = self.memory[self.program_counter as usize];
            self.program_counter += 1;
            match opc{
                //BRK
                0x00 => return,
                
                //LDY
                0xA0 => {
                    self.ldy(AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA4 => {
                    self.ldy(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xB4 => {
                    self.ldy(AddressingMode::ZeroPageX);
                    self.program_counter += 1;
                }
                0xAC => {
                    self.ldy(AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0xBC => {
                    self.ldy(AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                }
                
                //LDX
                0xA2 => {
                    self.ldx(AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA6 => {
                    self.ldx(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xB6 => {
                    self.ldx(AddressingMode::ZeroPageY);
                    self.program_counter += 1;
                }
                0xAE => {
                    self.ldx(AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0xBE => {
                    self.ldx(AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                }

                //LDA
                0xA9 => {
                    self.lda(AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xB5 => {
                    self.lda(AddressingMode::ZeroPageX);
                    self.program_counter += 1;
                }
                0xAD =>{
                    self.lda(AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0xBD => {
                    self.lda(AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                }
                0xB9 => {
                    self.lda(AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                }
                0xA1 => {
                    self.lda(AddressingMode::IndirectX);
                    self.program_counter += 1;
                }
                0xB1 => {
                    self.lda(AddressingMode::IndirectY);
                    self.program_counter += 1;
                }

                //STA
                0x85 => {
                    self.sta(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0x95 => {
                    self.sta(AddressingMode::ZeroPageX);
                    self.program_counter += 1;
                }
                0x8D => {
                    self.sta(AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0x9D => {
                    self.sta(AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                }
                0x99 => {
                    self.sta(AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                }
                0x81 => {
                    self.sta(AddressingMode::IndirectX);
                    self.program_counter += 1;
                }
                0x91 => {
                    self.sta(AddressingMode::IndirectY);
                    self.program_counter += 1;
                }

                //STX
                0x86 => {
                    self.stx(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0x96 => {
                    self.stx(AddressingMode::ZeroPageY);
                    self.program_counter += 1;
                }
                0x8E => {
                    self.stx(AddressingMode::Absolute);
                    self.program_counter += 2;
                }

                //STY
                0x84 => {
                    self.sty(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0x94 => {
                    self.sty(AddressingMode::ZeroPageY);
                    self.program_counter += 1;
                }
                0x8C => {
                    self.sty(AddressingMode::Absolute);
                    self.program_counter += 2;
                }

                //TAX
                0xAA => self.tax(),
                
                //INX
                0xE8 => self.inx(),
                
                //INY
                0xC8 => self.iny(),
                _ => todo!()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::vec;
    use super::*;

    #[test]
    fn test_0xa9_ldy_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.reg_y, 0x05);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status_reg & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.load(vec![0xaa, 0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_x, 10);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0;
        cpu.load_and_run(vec![0xaa,0x00]);
        assert!(cpu.status_reg & 0b0000_0010 == 0b10)
    }

    #[test]
    fn test_0xe8_inx_increment_x() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x15;
        cpu.load(vec![0xe8,0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_x, 0x16);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xc8_iny_increment_y() {
        let mut cpu = CPU::new();
        cpu.reg_y = 0x15;
        cpu.load(vec![0xc8,0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_y, 0x16);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }

    #[test]
    fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.reg_x, 0xc1)
}

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_iny_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_y = 0xff;
        cpu.load(vec![0xc8, 0xc8, 0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();

        assert_eq!(cpu.reg_y, 1)
    }

    #[test]
    fn test_write_mem_16bit(){
        let mut cpu = CPU::new();
        cpu.write_mem_u16(0x1FFF, 0xFA6E);
        assert!(cpu.memory[0x1FFF] == 0x6E);
        assert!(cpu.memory[0x2000] == 0xFA);
    }

    #[test]
    fn test_read_mem_16bit(){
        let mut cpu = CPU::new();
        cpu.memory[0x1FFF] = 0xAA;
        cpu.memory[0x2000] = 0xB3;
        assert!(cpu.read_mem_u16(0x1FFF) == 0xB3AA);
    }
     
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status_reg & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_lda_zero_page() {
       let mut cpu = CPU::new();
       cpu.write_mem(0x10, 0x55);

       cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

       assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_zero_page_x(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x10, 0x55);
        cpu.load_and_run(vec![0xa2, 0x01, 0xb5, 0x0f, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_absolute(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x10ae, 0x55);
        cpu.load_and_run(vec![0xad, 0xae, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.write_mem(0x10f1, 0x55);
        cpu.load_and_run(vec![0xa2, 0x01, 0xbd, 0xf0, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = CPU::new();
        cpu.write_mem(0x10f1, 0x55);
        cpu.load_and_run(vec![0xa0, 0x01, 0xb9, 0xf0, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = CPU::new();
        cpu.write_mem(0x10f1, 0x55);
        cpu.write_mem(0x00f1, 0xf1);
        cpu.write_mem(0x00f2, 0x10);
        cpu.load_and_run(vec![0xa2, 0x01, 0xa1, 0xf0, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_indirect_y(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x10f1, 0x55);
        cpu.write_mem(0x00f0, 0xf0);
        cpu.write_mem(0x00f1, 0x10);
        cpu.load_and_run(vec![0xa0, 0x01, 0xb1, 0xf0, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_0xa9_ldx_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.reg_x, 0x05);
        assert!(cpu.status_reg & 0b0000_0010 == 0b00);
        assert!(cpu.status_reg & 0b1000_0000 == 0);
    }
 
    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status_reg & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_ldx_zero_page(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x10, 0x55);
 
        cpu.load_and_run(vec![0xa6, 0x10, 0x00]);
 
        assert_eq!(cpu.reg_x, 0x55);
    }

    #[test]
    fn test_sta_0x85(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0x85, 0x02, 0x00]);

        assert!(cpu.memory[0x02] == 0x01)
    }

    #[test]
    fn test_stx_0x86(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x01, 0x86, 0x02, 0x00]);

        assert!(cpu.memory[0x02] == 0x01)
    }


    #[test]
    fn test_sty_0x84(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x01, 0x84, 0x02, 0x00]);

        assert!(cpu.memory[0x02] == 0x01)
    }
}