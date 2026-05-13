use esp_rom_sys::rom::md5;

#[allow(dead_code)]
pub struct Crc32 {}

impl Crc32 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub fn crc(&self, data: &[u8]) -> u32 {
        esp_rom_sys::rom::crc::crc32_le(u32::MAX, data)
    }
}

pub struct Md5 {
    context: md5::Context,
}

impl Md5 {
    pub fn new() -> Self {
        Self {
            context: md5::Context::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.context.consume(data);
    }

    pub fn finalize(self) -> [u8; 16] {
        self.context.compute().0
    }
}
