/*use nes_emulator::snake;*/
use nes_emulator::cartridge;
use nes_emulator::cpu::CPU;
use nes_emulator::bus;
use nes_emulator::log;
//use nes_emulator::snake;

fn main() {
    let bytes: Vec<u8> = std::fs::read("./games/nestest.nes").unwrap();
    let rom = cartridge::Rom::new(&bytes).unwrap();
    let mut cpu = CPU::new(bus::Bus::new(rom));
    cpu.reset();
    cpu.program_counter = 0xC000;
    cpu.run_with_callback(move|cpu|{
        println!("{}",log::log(cpu));
    });
}