use crate::opcodes;
extern crate bitflags;

bitflags::bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

pub struct CPU{
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    stack_ptr: u8,
    status_reg: CpuFlags, // NEG, OVERFLOW, B-flag, DECIMAL, INTERRUPT DISABLE, ZERO, CARRY
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

const STACK_RESET:u16 = 0x1FF;

impl CPU{
    pub fn new() -> Self{
        CPU{
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            stack_ptr: STACK_RESET as u8,
            status_reg: CpuFlags::empty(),
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
    
    fn update_z_and_neg_flag(&mut self,val:u8) {
        if val == 0 {
            self.status_reg.insert(CpuFlags::ZERO);
        } else {
            self.status_reg.remove(CpuFlags::ZERO);
        }

        if val & 0b1000_0000 != 0 {
            self.status_reg.insert(CpuFlags::NEGATIV);
        } else {
            self.status_reg .remove(CpuFlags::NEGATIV);
        }
    }

    pub fn read_mem(&mut self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    fn read_mem_u16(&mut self, addr: u16) -> u16{
        let lo = self.read_mem(addr) as u16;
        let hi = self.read_mem(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub fn write_mem(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }

    fn write_mem_u16(&mut self, addr: u16, data: u16){
        let lo = data & 0x00FF;
        let hi = (data & 0xFF00) >> 8;
        self.write_mem(addr, lo as u8);
        self.write_mem(addr + 1, hi as u8);
    }

    fn push_stack(&mut self, data: u8){
        self.write_mem(0x0100 + self.stack_ptr as u16, data);
        self.stack_ptr -= 1;
    }

    fn push_stack_u16(&mut self, data: u16){
        self.write_mem_u16(0x0100 + self.stack_ptr as u16, data);
        self.stack_ptr -= 2;
    }

    fn pop_stack(&mut self) -> u8{
        self.stack_ptr += 1;
        self.read_mem(0x0100 + self.stack_ptr as u16)
    }

    fn pop_stack_u16(&mut self) -> u16{
        self.stack_ptr += 2;
        self.read_mem_u16(0x0100 + self.stack_ptr as u16)
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(&mode);
        self.reg_a = self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_a);
    }

    fn ldx(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.reg_x = self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_x);
    }

    fn ldy(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.reg_y = self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_y);
    }

    fn tax(&mut self){
        self.reg_x = self.reg_a;
        self.update_z_and_neg_flag(self.reg_x)
    }

    fn inc(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let data = self.read_mem(addr).wrapping_add(1);
        self.write_mem(addr, data);
        self.update_z_and_neg_flag(data);
    }

    fn inx(&mut self){
        if self.reg_x == 0xFF{
            self.reg_x = 0;
        }else{
            self.reg_x += 1;
        }
        self.update_z_and_neg_flag(self.reg_x)
    }
    
    fn iny(&mut self){
        if self.reg_y == 0xFF{
            self.reg_y = 0;
        }else{
            self.reg_y += 1;
        }
        self.update_z_and_neg_flag(self.reg_y)
    }

    fn dec(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let data = self.read_mem(addr).wrapping_sub(1);
        self.write_mem(addr, data);
        self.update_z_and_neg_flag(data);
    }

    fn dex(&mut self){
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_z_and_neg_flag(self.reg_x);
    }

    fn dey(&mut self){
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_z_and_neg_flag(self.reg_y);
    }

    fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.write_mem( addr, self.reg_a);
    }

    fn stx(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.write_mem( addr, self.reg_x);
    }

    fn sty(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.write_mem( addr, self.reg_y);
    }

    fn add_to_a(&mut self, m: u8){
        let m = m as u16;
        let n = self.reg_a as u16;
        let r = m + n + (if self.status_reg.contains(CpuFlags::CARRY){
            1
        }else{
            0
        });
        self.update_z_and_neg_flag(r as u8);
        
        // set overflow flag if overflow occurs
        if (m ^r) & (n^r) & 0x80 != 0{
            self.status_reg.insert(CpuFlags::OVERFLOW);
        }else{
            self.status_reg.remove(CpuFlags::OVERFLOW);
        }
        //set c flag
        if r > 0xff{
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        self.reg_a = r as u8;
    }
    
    fn sbc(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        let data = self.read_mem(addr);
        self.add_to_a(((data ^ 0xff) + 1) as u8)

    }
    
    fn adc(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        let data = self.read_mem(addr);
        self.add_to_a(data);
    }
    
    fn asl(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let data = self.read_mem(addr);
        self.write_mem(addr, data << 1);
        if data & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        self.update_z_and_neg_flag(data << 1);
    }

    fn lsr(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let data = self.read_mem(addr);
        self.write_mem(addr, data >> 1);
        if data & 0b0000_0001 != 0{
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        self.update_z_and_neg_flag(data >> 1);
    }

    fn rol(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let mut data = self.read_mem(addr);
        let c = self.status_reg.contains(CpuFlags::CARRY);
        if data & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        data = data << 1;
        if c{
            data += 1;
        }
        self.write_mem(addr, data);
        self.update_z_and_neg_flag(data );
    }

    fn ror(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let mut data = self.read_mem(addr);
        let c = self.status_reg.contains(CpuFlags::CARRY);
        if data & 0b0000_0001 != 0{
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        data = data >> 1;
        if c{
            data += 0b1000_0000;
        }
        self.write_mem(addr, data);
        self.update_z_and_neg_flag(data );
    }

    fn and(&mut self, mode: &AddressingMode){
        let addr = self.get_address(&mode);
        self.reg_a = self.reg_a & self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_a);
    }

    fn eor(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        self.reg_a = self.reg_a ^ self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_a);
    }

    fn ora(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        self.reg_a = self.reg_a | self.read_mem(addr);
        self.update_z_and_neg_flag(self.reg_a);
    }

    fn bcc(&mut self){
        if !self.status_reg.contains(CpuFlags::CARRY) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn bcs(&mut self){
        if self.status_reg.contains(CpuFlags::CARRY) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn beq(&mut self){
        if self.status_reg.contains(CpuFlags::ZERO) {
            println!("took branch");
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            println!("didnt take branch");
            self.program_counter += 1;
        }
    }

    fn bne(&mut self){
        if !self.status_reg.contains(CpuFlags::ZERO) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn bmi(&mut self){
        if self.status_reg.contains(CpuFlags::NEGATIV) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }
    
    fn bpl(&mut self){
        if !self.status_reg.contains(CpuFlags::NEGATIV) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn bvc(&mut self){
        if !self.status_reg.contains(CpuFlags::OVERFLOW) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn bvs(&mut self){
        if self.status_reg.contains(CpuFlags::OVERFLOW) {
            let offset = self.read_mem(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(offset as u16).wrapping_add(1);
        }else{
            self.program_counter += 1;
        }
    }

    fn bit(&mut self, mode: &AddressingMode){
        let addr = self.get_address(mode);
        let r = self.reg_a & self.read_mem(addr);
        if r == 0 {
            self.status_reg.insert(CpuFlags::ZERO);
        }
        else{
            self.status_reg.remove(CpuFlags::ZERO);
        }
        if r & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::NEGATIV);
        }else{
            self.status_reg.remove(CpuFlags::NEGATIV);
        }
        if r & 0b0100_0000 != 0{
            self.status_reg.insert(CpuFlags::OVERFLOW);
        }else{
            self.status_reg.remove(CpuFlags::OVERFLOW);
        }
    }
    
    fn cmp(&mut self,mode: &AddressingMode){
        let addr = self.get_address(mode);
        let r = self.reg_a.wrapping_sub(self.read_mem(addr));
        if self.reg_a >= self.read_mem(addr){
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        if r == 0{
            self.status_reg.insert(CpuFlags::ZERO);
        }else{
            self.status_reg.remove(CpuFlags::ZERO);
        }
        if r & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::NEGATIV);
        }else{
            self.status_reg.remove(CpuFlags::NEGATIV);
        }
    }

    fn cpx(&mut self,mode: &AddressingMode){
        let addr = self.get_address(mode);
        let r = self.reg_x.wrapping_sub(self.read_mem(addr));
        if self.reg_x >= self.read_mem(addr){
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        if r == 0{
            self.status_reg.insert(CpuFlags::ZERO);
        }else{
            self.status_reg.remove(CpuFlags::ZERO);
        }
        if r & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::NEGATIV);
        }else{
            self.status_reg.remove(CpuFlags::NEGATIV);
        }
    }

    fn cpy(&mut self,mode: &AddressingMode){
        let addr = self.get_address(mode);
        let r = self.reg_y.wrapping_sub(self.read_mem(addr));
        if self.reg_y >= self.read_mem(addr){
            self.status_reg.insert(CpuFlags::CARRY);
        }else{
            self.status_reg.remove(CpuFlags::CARRY);
        }
        if r == 0{
            self.status_reg.insert(CpuFlags::ZERO);
        }else{
            self.status_reg.remove(CpuFlags::ZERO);
        }
        if r & 0b1000_0000 != 0{
            self.status_reg.insert(CpuFlags::NEGATIV);
        }else{
            self.status_reg.remove(CpuFlags::NEGATIV);
        }
    }

    fn jsr(&mut self){
        self.push_stack_u16(self.program_counter + 2);
        self.program_counter = self.read_mem_u16(self.program_counter);
    }

    fn rts(&mut self){
        self.program_counter = self.pop_stack_u16();
    }
    
    fn rti(&mut self){
        self.status_reg.bits = self.pop_stack();
        self.status_reg.remove(CpuFlags::BREAK);
        self.status_reg.remove(CpuFlags::BREAK2);
        self.program_counter = self.pop_stack_u16();
    }
    //whipes all registers and sets program counter to addr stored at 0xFFFC
    pub fn reset(&mut self){
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.status_reg = CpuFlags::from_bits_truncate(0b100100);
        self.program_counter = self.read_mem_u16(0xFFFC);
    }

    //load progarm to memory an stores starting address
    pub fn load(&mut self, program: Vec<u8>){
        self.memory[0x0600..(0x0600+program.len())].copy_from_slice(&program);
        self.write_mem_u16(0xFFFC,0x0600);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self){
        self.run_with_callback(|_|{});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)where F: FnMut(&mut CPU),
        {
        loop {
            callback(self);
            let opc = self.read_mem(self.program_counter);
            println!("run: {:x?}: {}: {:x?}",self.program_counter, opcodes::OP_MAP.get(&opc).unwrap().name, opc);
            self.program_counter += 1;
            match opc{
                //BRK
                0x00 => {
                    self.status_reg.insert(CpuFlags::BREAK);
                    return;
                }
                
                //NOP
                0xEA => continue,

                //ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 =>{
                    let op = *opcodes::OP_MAP.get(&opc).unwrap();
                    self.adc(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 =>{
                    let op = opcodes::OP_MAP.get(&opc).unwrap();
                    self.sbc(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //ASL
                0x0A => {
                    if self.reg_a & 0b1000_0000 != 0{
                        self.status_reg.insert(CpuFlags::CARRY);
                    }else{
                        self.status_reg.remove(CpuFlags::CARRY);
                    }
                    self.reg_a = self.reg_a << 1;
                    self.update_z_and_neg_flag(self.reg_a);
                }
                0x06 | 0x16 | 0x0E | 0x1E => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.asl(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //LSR
                0x4A => {
                    if self.reg_a & 0b0000_0001 != 0{
                        self.status_reg.insert(CpuFlags::CARRY);
                    }else{
                        self.status_reg.remove(CpuFlags::CARRY);
                    }
                    self.reg_a = self.reg_a >> 1;
                    self.update_z_and_neg_flag(self.reg_a);
                }
                0x46 | 0x56 | 0x4E | 0x5E => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.lsr(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //ROL
                0x2A => {
                    let c = self.status_reg.contains(CpuFlags::CARRY);
                    if self.reg_a & 0b1000_0000 != 0{
                        self.status_reg.insert(CpuFlags::CARRY);
                    }else{
                        self.status_reg.remove(CpuFlags::CARRY);
                    }
                    self.reg_a = self.reg_a << 1;
                    if c{
                        self.reg_a += 1;
                    }
                    self.update_z_and_neg_flag(self.reg_a);
                }
                0x26 | 0x36 | 0x2E | 0x3E => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.rol(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //ROR
                0x6A => {
                    let c = self.status_reg.contains(CpuFlags::CARRY);
                    if self.reg_a & 0b0000_0001 != 0{
                        self.status_reg.insert(CpuFlags::CARRY);
                    }else{
                        self.status_reg.remove(CpuFlags::CARRY);
                    }
                    self.reg_a = self.reg_a >> 1;
                    if c{
                        self.reg_a += 0b1000_0000;
                    }
                    self.update_z_and_neg_flag(self.reg_a);
                }
                0x66 | 0x76 | 0x6E | 0x7E => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.ror(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    let op = opcodes::OP_MAP.get(&opc).unwrap();
                    self.and(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //EOR
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 =>{
                    let op = opcodes::OP_MAP.get(&opc).unwrap();
                    self.eor(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //ORA
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 =>{
                    let op = opcodes::OP_MAP.get(&opc).unwrap();
                    self.ora(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //CLC
                0x18 => self.status_reg.remove(CpuFlags::CARRY),

                //SEC
                0x38 => self.status_reg.insert(CpuFlags::CARRY),

                //CLD
                0xD8 => self.status_reg.remove(CpuFlags::DECIMAL_MODE),

                //SED
                0xF8 => self.status_reg.insert(CpuFlags::DECIMAL_MODE),

                //CLI
                0x58 => self.status_reg.remove(CpuFlags::INTERRUPT_DISABLE),

                //SEI
                0x78 => self.status_reg.insert(CpuFlags::INTERRUPT_DISABLE),

                //CLV
                0xB8 => self.status_reg.remove(CpuFlags::OVERFLOW),

                //CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.cmp(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //CPX
                0xE0 | 0xE4 | 0xEC => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.cpx(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //CPY
                0xC0 | 0xC4 | 0xCC => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.cpy(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                
                //BCC
                0x90 => self.bcc(),

                //BCS
                0xB0 => self.bcs(),

                //BEQ
                0xF0 => self.beq(),

                //BNE
                0xD0 => self.bne(),

                //BMI
                0x30 => self.bmi(),

                //BPL 
                0x10 => self.bpl(),

                //BVC
                0x50 => self.bvc(),

                //BVS
                0x70 => self.bvs(),

                //BIT
                0x24 | 0x2C => {
                    let op = opcodes::OP_MAP.get(&opc).unwrap();
                    self.bit(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                
                //JMP
                0x4C => {
                    self.program_counter = self.read_mem_u16(self.program_counter);
                }
                0x6C => {
                    let addr = self.read_mem(self.program_counter) as u16;
                    self.program_counter = self.read_mem_u16(addr);
                }

                //RTI
                0x40 => self.rti(),

                //RTS
                0x60 => self.rts(),

                //JSR
                0x20 => self.jsr(),

                //PHA
                0x48 => self.push_stack(self.reg_a),

                //PLA
                0x68 => {
                    self.reg_a = self.pop_stack();
                    self.update_z_and_neg_flag(self.reg_a);
                }

                //PHP
                0x08 => {
                    self.status_reg.insert(CpuFlags::BREAK);
                    self.status_reg.insert(CpuFlags::BREAK2);
                    self.push_stack(self.status_reg.bits());
                }
                
                //PLP
                0x28 => {
                    self.status_reg.bits = self.pop_stack();
                    self.status_reg.remove(CpuFlags::BREAK);
                    self.status_reg.remove(CpuFlags::BREAK2);
                }

                //LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC=> {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.ldy(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }
                
                //LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.ldx(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.lda(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.sta(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //STX
                0x86 | 0x96 | 0x8E => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.stx(&op.addr_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }

                //STY
                0x84 | 0x94 | 0x8C=> {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self.sty(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //TAX
                0xAA => self.tax(),
                
                //TAY
                0xA8 => {
                    self.reg_y = self.reg_a;
                    self.update_z_and_neg_flag(self.reg_y);
                }

                //TSX
                0xBA => {
                    self.reg_x = self.stack_ptr;
                    self.update_z_and_neg_flag(self.reg_x);
                }

                //TXA
                0x8A => {
                    self.reg_a = self.reg_x;
                    self.update_z_and_neg_flag(self.reg_a);
                }

                //TXS
                0x9A => self.stack_ptr = self.reg_x,

                //TYA
                0x98 => {
                    self.reg_a = self.reg_y;
                    self.update_z_and_neg_flag(self.reg_a);
                }

                //INC
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self. inc(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //INX
                0xE8 => self.inx(),
                
                //INY
                0xC8 => self.iny(),

                //DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    let op = &**opcodes::OP_MAP.get(&opc).unwrap();
                    self. dec(&op.addr_mode);
                    self.program_counter += op.bytes as u16 -1;
                }

                //DEX
                0xCA => self.dex(),

                //DEY
                0x88 => self.dey(),

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
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_0xa9_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.load(vec![0xaa, 0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_x, 10);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0;
        cpu.load_and_run(vec![0xaa,0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xe8_inx_increment_x() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x15;
        cpu.load(vec![0xe8,0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_x, 0x16);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_0xc8_iny_increment_y() {
        let mut cpu = CPU::new();
        cpu.reg_y = 0x15;
        cpu.load(vec![0xc8,0x00]);
        cpu.program_counter = 0x8000;
        cpu.run();
        assert_eq!(cpu.reg_y, 0x16);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
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
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
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
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
    }
 
    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
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

    #[test]
    fn test_adc_for_cout(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x0001, 0xff);
        cpu.load_and_run(vec![0xa9, 0x01, 0x65, 0x01, 0x00]);
        assert!(cpu.reg_a == 0x00);
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_for_oveflow_detection1(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x0001, 0x50);
        cpu.load_and_run(vec![0xa9, 0x50, 0x65, 0x01, 0x00]);
        assert!(cpu.reg_a == 0xa0);
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_for_oveflow_detection2(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x0001, 0xd0);
        cpu.load_and_run(vec![0xa9, 0x90, 0x65, 0x01, 0x00]);
        assert!(cpu.reg_a == 0x60);
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_for_oveflow_detection3(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x0001, 0xd0);
        cpu.load_and_run(vec![0xa9, 0x90, 0x65, 0x01, 0x00]);
        assert!(cpu.reg_a == 0x60);
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_sbc_for_oveflow_detection1(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x0001, 0xb0);
        cpu.load_and_run(vec![0xa9,0x50,0xe5,0x01, 0x00]);
        assert_eq!(cpu.reg_a, 0xa0);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_sbc_for_oveflow_detection2(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x0001, 0x70);
        cpu.load_and_run(vec![0xa9,0xd0,0xe5,0x01, 0x00]);
        assert_eq!(cpu.reg_a, 0x60);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_sbc_for_oveflow_detection3(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x0001, 0xf0);
        cpu.load_and_run(vec![0xa9,0x50,0xe5,0x01, 0x00]);
        assert_eq!(cpu.reg_a, 0x60);
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_and(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b1001_0101,0x29,0b1101_1011, 0x00]);
        assert_eq!(cpu.reg_a, 0b1001_0001);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_bit1(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0b1101_0110);
        cpu.load_and_run(vec![0xa9, 0b1101_0101, 0x24, 0x01, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_bit2(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0b1001_0110);
        cpu.load_and_run(vec![0xa9, 0b0110_0001, 0x24, 0x01, 0x00]);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_cmp1(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xc9, 0xfe, 0x00]);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_cmp2(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xfe, 0xc9, 0xfe, 0x00]);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_cmp3(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xfd, 0xc9, 0xfe, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_cpx(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xfe, 0xe0, 0xff, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_cpy(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xfe, 0xc0, 0xff, 0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_dex(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0xca, 0x00]);
        assert_eq!(cpu.reg_x, 0xff);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_dey(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x88, 0x00]);
        assert_eq!(cpu.reg_y, 0xff);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }
    
    #[test]
    fn test_dec1(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0x00);
        cpu.load_and_run(vec![0xC6, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0xff);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_dec2(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0x01);
        cpu.load_and_run(vec![0xC6, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0x00);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_inc1(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0xff);
        cpu.load_and_run(vec![0xE6, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0x00);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_inc2(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x01, 0xfe);
        cpu.load_and_run(vec![0xE6, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0xff);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_push_pop_stack(){
        let mut  cpu = CPU::new();
        cpu.push_stack(0xa1);
        assert_eq!(cpu.pop_stack(), 0xa1);
    }

    #[test]
    fn test_push_pop_stack_u16(){
        let mut  cpu = CPU::new();
        cpu.push_stack_u16(0xa1b8);
        assert_eq!(cpu.pop_stack_u16(), 0xa1b8);
    }

    #[test]
    fn test_eor(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b1001_0101,0x49,0b0101_1011, 0x00]);
        assert_eq!(cpu.reg_a, 0b1100_1110);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_php_plp(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x38,0x08,0x18,0x28,0x00]);
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_asl1(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b0101_0011,0x0a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1010_0110);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_asl2(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x01, 0b0101_0011);
        cpu.load_and_run(vec![0x06,0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0b1010_0110);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_lsr1(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b0101_0011,0x4a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0010_1001);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_lsr2(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x01, 0b0101_0011);
        cpu.load_and_run(vec![0x46,0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0b0010_1001);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_rol1(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b0101_0011,0x02a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1010_0110);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_rol2(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x01, 0b1101_0011);
        cpu.load_and_run(vec![0x38, 0x26, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0b1010_0111);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
    }    

    #[test]
    fn test_ror1(){
        let mut  cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0b0101_0010,0x06a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0010_1001);
        assert!(!cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(!cpu.status_reg.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_ror2(){
        let mut  cpu = CPU::new();
        cpu.write_mem(0x01, 0b1101_0011);
        cpu.load_and_run(vec![0x38, 0x66, 0x01, 0x00]);
        assert_eq!(cpu.read_mem(0x01), 0b1110_1001);
        assert!(cpu.status_reg.contains(CpuFlags::NEGATIV));
        assert!(!cpu.status_reg.contains(CpuFlags::ZERO));
        assert!(cpu.status_reg.contains(CpuFlags::CARRY));
    }    

    #[test]
    fn test_subroutines(){
        let mut cpu = CPU::new();
        cpu.write_mem(0x8020, 0xc8);
        cpu.write_mem(0x8021, 0x60);
        cpu.load_and_run(vec![0xa0,0x10,0x20,0x20,0x80,0x00]);
        assert_eq!(cpu.reg_y, 0x11);
    }

    #[test]
    fn test_branching(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0,0x10,0x38,0xb0,0x02,0x00,0x00,0xc8,0x00]);
        assert_eq!(cpu.reg_y, 0x11);
    }

}