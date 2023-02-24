use crate::cpu::{CPU,AddressingMode};
use crate::opcodes;

pub fn log(cpu: &mut CPU) -> String{
    let pc = cpu.program_counter; 
    let opc = cpu.read_mem(pc);
    let op = opcodes::OP_MAP.get(&opc).unwrap();
    
    let mut hexdump:Vec<u8> = vec![];
    let mut tmp = String::new();
    hexdump.push(opc);
    match op.bytes {
        1 => {
            if op.name == "ROL" || op.name == "ROR" || op.name == "LSR" || op.name == "ASL"{
                tmp.push('A');
            }
        }
        2 => {
            match op.addr_mode{
                AddressingMode::Immediate => {
                    tmp.push_str(&format!("#${:02x}",cpu.read_mem(pc + 1)));
                    hexdump.push(cpu.read_mem(pc + 1));
                }
                AddressingMode::NoneAddressing => {
                    hexdump.push(cpu.read_mem(pc + 1));
                    tmp.push_str(&format!("${:02x}",(pc as usize + 2).wrapping_add((cpu.read_mem(pc + 1) as i8) as usize)))
                },
                AddressingMode::ZeroPage => {
                    let  addr = cpu.read_mem(pc + 1);
                    tmp.push_str(&format!("${:02x} = {:02x}",addr, cpu.read_mem(addr as u16)));
                    hexdump.push(cpu.read_mem(pc + 1));
                }
                AddressingMode::ZeroPageX => {
                    let addr = cpu.read_mem(pc + 1);
                    tmp.push_str(&format!("${:02x},X @ {:02x} = {:02x}",
                    addr, 
                    addr.wrapping_add(cpu.reg_x), 
                    cpu.read_mem(addr.wrapping_add(cpu.reg_x) as u16)));
                    hexdump.push(cpu.read_mem(pc + 1));
                }
                AddressingMode::ZeroPageY => {
                    let addr = cpu.read_mem(pc + 1);
                    tmp.push_str(&format!("${:02x},Y @ {:02x} = {:02x}",
                    addr, 
                    addr.wrapping_add(cpu.reg_y), 
                    cpu.read_mem(addr.wrapping_add(cpu.reg_y) as u16)));
                    hexdump.push(cpu.read_mem(pc + 1));
                }
                AddressingMode::IndirectX => {
                    let addr = cpu.read_mem(pc + 1);
                    let lo = cpu.read_mem(addr.wrapping_add(cpu.reg_x) as u16) as u16;
                    let hi = cpu.read_mem(addr.wrapping_add(cpu.reg_x).wrapping_add(1) as u16) as u16;
                    let real_addr = hi << 8 | lo;
                    tmp.push_str(&format!("(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    addr, 
                    addr.wrapping_add(cpu.reg_x), 
                    real_addr,
                    cpu.read_mem(real_addr)));
                    hexdump.push(cpu.read_mem(pc + 1));
                }
                AddressingMode::IndirectY => {
                    let addr = cpu.read_mem(pc + 1);
                    let lo = cpu.read_mem(addr as u16) as u16;
                    let hi = cpu.read_mem(addr.wrapping_add(1) as u16) as u16;
                    let pt_addr = hi << 8 | lo;
                    tmp.push_str(&format!("(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    addr, 
                    pt_addr, 
                    pt_addr.wrapping_add(cpu.reg_y as u16),
                    cpu.read_mem(pt_addr.wrapping_add(cpu.reg_y as u16))));
                    hexdump.push(cpu.read_mem(pc + 1));
                }

                _ => panic!("Invalid operation")
            }
        }
        3 => {
            match op.addr_mode{
                AddressingMode::NoneAddressing => {}
                AddressingMode::Absolute =>{
                    hexdump.push(cpu.read_mem(pc + 1));
                    hexdump.push(cpu.read_mem(pc + 2));
                    let addr = cpu.read_mem_u16(pc + 1);
                    if op.name == "JMP" || op.name == "JSR"{
                        tmp.push_str(&format!("${:04x}",addr));
                    }else{
                        tmp.push_str(&format!("${:04x} = {:02x}",addr, cpu.read_mem(addr as u16)));
                    }
                }
                AddressingMode::AbsoluteX =>{
                    let addr = cpu.read_mem_u16(pc + 1);
                    tmp.push_str(&format!("${:04x},X @ {:04x} = {:02x}",addr, addr.wrapping_add(cpu.reg_x as u16), cpu.read_mem((addr as u16).wrapping_add(cpu.reg_x as u16))));
                    hexdump.push(cpu.read_mem(pc + 1));
                    hexdump.push(cpu.read_mem(pc + 2));
                }
                AddressingMode::AbsoluteY =>{
                    let addr = cpu.read_mem_u16(pc + 1);
                    tmp.push_str(&format!("${:04x},Y @ {:04x} = {:02x}",addr, addr.wrapping_add(cpu.reg_y as u16), cpu.read_mem((addr as u16).wrapping_add(cpu.reg_y as u16))));
                    hexdump.push(cpu.read_mem(pc + 1));
                    hexdump.push(cpu.read_mem(pc + 2));
                }
                AddressingMode::IndirectX => {
                    let addr = cpu.read_mem_u16(pc + 1);
                    let jmp_addr = if addr & 0x00FF == 0x00FF {
                        let lo = cpu.read_mem(addr);
                        let hi = cpu.read_mem(addr & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    } else {
                        cpu.read_mem_u16(addr)
                    };
                    hexdump.push(cpu.read_mem(pc + 1));
                    hexdump.push(cpu.read_mem(pc + 2));
                    // let jmp_addr = cpu.mem_read_u16(address);
                    tmp.push_str(&format!("(${:04x}) = {:04x}", addr, jmp_addr));
                }
                _ => panic!("Invalid operation")
            }

        }
        _ => panic!("Invalid opcode: {:?}", opc),
    }


    let hex_str = hexdump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8} {: >4} {}", pc, hex_str, op.name, tmp)
        .trim()
        .to_string();
    format!("{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}", asm_str, cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.status_reg.bits(), cpu.stack_ptr).to_ascii_uppercase()
} 

#[cfg(test)]
mod test {
   use super::*;
   use crate::bus::Bus;
   use crate::cartridge::test::test_rom;

   #[test]
   fn test_format_trace() {
       let mut bus = Bus::new(test_rom());
       bus.write_mem(100, 0xa2);
       bus.write_mem(101, 0x01);
       bus.write_mem(102, 0xca);
       bus.write_mem(103, 0x88);
       bus.write_mem(104, 0x00);

       let mut cpu = CPU::new(bus);
       cpu.program_counter = 0x64;
       cpu.reg_a = 1;
       cpu.reg_x = 2;
       cpu.reg_y = 3;
       let mut result: Vec<String> = vec![];
       cpu.run_with_callback(|cpu| {
           result.push(log(cpu));
       });
       assert_eq!(
           "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
           result[0]
       );
       assert_eq!(
           "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
           result[1]
       );
       assert_eq!(
           "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
           result[2]
       );
   }

   #[test]
   fn test_format_mem_access() {
       let mut bus = Bus::new(test_rom());
       // ORA ($33), Y
       bus.write_mem(100, 0x11);
       bus.write_mem(101, 0x33);


       //data
       bus.write_mem(0x33, 00);
       bus.write_mem(0x34, 04);

       //target cell
       bus.write_mem(0x400, 0xAA);

       let mut cpu = CPU::new(bus);
       cpu.program_counter = 0x64;
       cpu.reg_y = 0;
       let mut result: Vec<String> = vec![];
       cpu.run_with_callback(|cpu| {
           result.push(log(cpu));
       });
       assert_eq!(
           "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
           result[0]
       );
   }
}
