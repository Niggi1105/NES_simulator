extern crate lazy_static;
use std::collections::HashMap;
use crate::cpu::AddressingMode;

pub struct OpCode{
    pub op: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub addr_mode: AddressingMode,
}
impl OpCode {
    pub fn new(opc: u8, na: &'static str, bt: u8, cy: u8, admode: AddressingMode) -> Self {
        OpCode { op: opc, name: na, bytes: bt, cycles: cy, addr_mode: admode}
    }

}
lazy_static::lazy_static!{
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),     //break

        OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing),     //No Operation
        
        OpCode::new(0x90, "BCC", 2, 2, AddressingMode::NoneAddressing),     //branch if carry clear
        OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::NoneAddressing),     //branch if carry set
        OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::NoneAddressing),     //branch if equal (zero flag)
        OpCode::new(0x30, "BMI", 2, 2, AddressingMode::NoneAddressing),     //branch if negative 
        OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::NoneAddressing),     //branch if not equal
        OpCode::new(0x10, "BPL", 2, 2, AddressingMode::NoneAddressing),     //branch if positive
        OpCode::new(0x50, "BVC", 2, 2, AddressingMode::NoneAddressing),     //branch if overflow flag clear
        OpCode::new(0x70, "BVS", 2, 2, AddressingMode::NoneAddressing),     //branch if overflow flag set

        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),     //clear carry flag
        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),     //clear Decimal Mode
        OpCode::new(0x50, "CLI", 1, 2, AddressingMode::NoneAddressing),     //clear Interrupt disable
        OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing),     //clear Overflow flag

        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),     //set carry flag
        OpCode::new(0xF8, "SED", 1, 2, AddressingMode::NoneAddressing),     //set Decimal Mode
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),     //set Interrupt disable

        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),     //transfer a to x
        OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),     //transfer a to y
        OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing),     //transfer stack pointer to x
        OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),     //transfer x to a
        OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing),     //transfer x to stack pointer
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),     //transfer y to a

        OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing),     //push accumulator to stack
        OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing),     //pull accumulator from stack
        OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing),     //push status reg to stack
        OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing),     //pull status reg from stack
        
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),     //increment x reg
        OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),     //increment y reg
        OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing),     //decrement x reg
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),     //decrement y reg

        OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),           //increment memory location
        OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xFE, "INC", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),           //decrement memory location
        OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),           //bit test
        OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),          //compare accumulator with memory
        OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xDD, "CMP", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0xD9, "CMP", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xD1, "CMP", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),          //compare x reg with memory
        OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),          //compare y reg with memory
        OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute),           //jump to Absolute or Indirect address
        OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::IndirectX),

        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),           //jump to subroutine, save return address to stack

        OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),          //load values into y reg
        OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBC, "LDY", 3, 4, AddressingMode::AbsoluteX),

        OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),          //load value into x reg
        OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBE, "LDX", 3, 4, AddressingMode::AbsoluteY),

        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),          //load value to Accumulator
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),           //store accumulator in memory
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),

        OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),           //store y reg in memory
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),           //store x reg in memory
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),          //add to accumulator with cin and cout
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),          //subtract from Accumulator with borrow
        OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xFD, "SBC", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0xF9, "SBC", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xF1, "SBC", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::NoneAddressing),     //shift left in memory or accumulator
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),        
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::NoneAddressing),     //shift right in memory or accumulator
        OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),        
        OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5E, "LSR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing),     //rotate left in memory or accumulator
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),        
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::NoneAddressing),     //rotate right in memory or accumulator
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),        
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7E, "ROR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),          //bitwise AND accumulator with memory
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3D, "AND", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, "AND", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0x49, "XOR", 2, 2, AddressingMode::Immediate),          //bitwise XOR accumulator with memory
        OpCode::new(0x45, "XOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "XOR", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x4D, "XOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5D, "XOR", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0x59, "XOR", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0x41, "XOR", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x51, "XOR", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),          //bitwise OR accumulator with memory
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1D, "ORA", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0x19, "ORA", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x11, "ORA", 2, 5, AddressingMode::IndirectY),
    ];

    pub static ref OP_MAP: HashMap<u8,&'static OpCode> = {
        let mut map = HashMap::new();
        for cpu_op in &*CPU_OPCODES{
            map.insert(cpu_op.op, cpu_op);
        }
        map
    };
}