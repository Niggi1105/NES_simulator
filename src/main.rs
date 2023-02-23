use nes_emulator::cpu;
fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0x00]);
    println!("Hello, world!");
}
