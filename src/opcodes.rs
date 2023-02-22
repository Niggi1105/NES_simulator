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

        
        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),     //transfer a to x
        
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),     //increment x reg
        OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),     //increment y reg


        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),           //bit test
        OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),

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

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),          //bitwise AND accumulator with memory
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3D, "AND", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, "AND", 2, 5, AddressingMode::IndirectY),
    ];

    pub static ref OP_MAP: HashMap<u8,&'static OpCode> = {
        let mut map = HashMap::new();
        for cpu_op in &*CPU_OPCODES{
            map.insert(cpu_op.op, cpu_op);
        }
        map
    };
}