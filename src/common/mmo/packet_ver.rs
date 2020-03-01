//! packet ver and related contants
//!

pub struct PacketVer(pub u32);

impl PacketVer {
    fn max_hotkeys(&self) -> usize {
        match self.0 {
            0..=20090602 => 27,
            20090603..=20090616 => 36,
            _ => 38,
        }
    }

    fn max_chars(&self) -> usize {
        match self.0 {
            0..=20100412 => 9,
            20100413..=20180124 => 12,
            _ => 15,
        }
    }
}
