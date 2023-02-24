/*use nes_emulator::snake;*/
use nes_emulator::cartridge;
use nes_emulator::cpu::CPU;
use nes_emulator::bus;

fn main() {
    let bytes: Vec<u8> = std::fs::read("./games/snake.nes").unwrap();
    let rom = cartridge::Rom::new(&bytes).unwrap();
    let mut cpu = CPU::new(bus::Bus::new(rom));
    cpu.reset();
}