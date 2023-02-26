pub struct ScrollReg{
    value: (u8,u8),
    hrz_pt: bool,
}

impl ScrollReg{
    pub fn new() -> Self{
        ScrollReg{
            value: (0,0),
            hrz_pt: true,
        }
    }

    pub fn write(&mut self, data:u8){
        if self.hrz_pt {
            self.value.0 = data;
            self.hrz_pt = false;
        }else{
            if data <= 239{
                self.value.1 = data;
            }else{
                self.value.1 = 239 - data;
            }

        }
    }

    pub fn reset_latch(&mut self){
        self.hrz_pt = true;
    }
}