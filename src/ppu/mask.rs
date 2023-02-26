use bitflags;

/*7  bit  0
---- ----
BGRs bMmG
|||| ||||
|||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
|||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
|||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
|||| +---- 1: Show background
|||+------ 1: Show sprites
||+------- Emphasize red (green on PAL/Dendy)
|+-------- Emphasize green (red on PAL/Dendy)
+--------- Emphasize blue
 */

bitflags::bitflags! {
    pub struct Mask: u8{
        const GREY_SCALE = 0b0000_0001;
        const BACKGROUND_LEFT = 0b0000_0010;
        const SPRITES_LEFT = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPH_RED = 0b0010_0000;
        const EMPH_GREEN = 0b010_0000;
        const EMPH_BLUE = 0b1000_0000;
    }
}
impl Mask{
    pub fn set_gsc(&mut self){
        self.insert(Mask::GREY_SCALE);
    }
    pub fn set_bcl(&mut self){
        self.insert(Mask::BACKGROUND_LEFT);
    }
    pub fn set_spl(&mut self){
        self.insert(Mask::SPRITES_LEFT);
    }
    pub fn set_sbg(&mut self){
        self.insert(Mask::SHOW_BACKGROUND);
    }
    pub fn set_ssp(&mut self){
        self.insert(Mask::SHOW_SPRITES);
    }
    pub fn set_emr(&mut self){
        self.insert(Mask::EMPH_GREEN);
    }
    pub fn set_emg(&mut self){
        self.insert(Mask::EMPH_GREEN);
    }
    pub fn set_emb(&mut self){
        self.insert(Mask::EMPH_BLUE);
    }
    pub fn set_mltpl(&mut self, m: u8){
        self.bits = m;
    }
}