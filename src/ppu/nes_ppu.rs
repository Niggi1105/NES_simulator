use crate::cartridge::Mirroring;
use crate::ppu::addr_reg::AddrRegister;
use crate::ppu::control_reg::ControlRegister;
use crate::ppu::mask::Mask;
use crate::ppu::status::Status;
use crate::ppu::scroll::ScrollReg;

pub struct PPU{
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    addr: AddrRegister,
    status: Status,
    pub mask: Mask,
    scroll: ScrollReg,
    data_buf: u8,
    oam_addr: u8,

}

//------------------------------------------------| 0xFFFF
//              Mirrors 0x0000 - 0x3FFF           | 
//                                                |
//------------------------------------------------| 0x4000
//                   Palletes                     |
//                                                |
//------------------------------------------------| 0x3F00                                                |
//               Name Tables (VRAM)               |
//                                                |
//------------------------------------------------| 0x2000                                            |
//            Pattern Tables (CHR ROM)            |
//                                                |
//------------------------------------------------| 0x0000


impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
            data_buf: 0,
            mask: Mask::empty(),
            status: Status::empty(),
            scroll: ScrollReg::new(),
            oam_addr: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }
    
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
 
    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();
        match addr {
            0..=0x1fff => {
                let result = self.data_buf;
                self.data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.data_buf;
                self.data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),
            
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_data(&mut self, data: u8){
        let addr = self.addr.get();
        self.increment_vram_addr();
        match addr {
            0..=0x1fff => {
                panic!("Unexpected write to chr ROM {:x}",addr);
            }
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = data;
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),

            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = data;
            }
            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize] = data;
            }
            _ => panic!("unexpected write to mirrored space {}", addr)
        }
    }

    pub fn read_status(&mut self) -> u8{
        let r = self.status.bits();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        self.status.clear_vblank();
        r
    }

    pub fn set_oam_addr(&mut self, addr: u8){
        self.oam_addr = addr;
    }

    pub fn read_oam_data(&mut self) -> u8{
        self.oam_data[self.oam_addr as usize]
    }

    pub fn write_oam_data(&mut self, value: u8){
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_scroll(&mut self, data: u8){
        self.scroll.write(data);
    }

    pub fn oam_dma(&mut self, data: &[u8]){
        for d in data{
            self.oam_data[self.oam_addr as usize] = *d;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        } 
    }


}