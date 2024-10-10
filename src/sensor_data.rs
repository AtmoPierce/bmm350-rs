use crate::types::{Sensor3DData, Sensor3DDataScaled};

impl Sensor3DData {
    /// Convert raw sensor data to scaled values in µT
    pub fn to_ut(&self, scale: f32) -> Sensor3DDataScaled {
        Sensor3DDataScaled {
            x: Self::lsb_to_ut(self.x, scale),
            y: Self::lsb_to_ut(self.y, scale),
            z: Self::lsb_to_ut(self.z, scale),
        }
    }

    /// Convert raw LSB value to µT
    fn lsb_to_ut(val: i16, scale: f32) -> f32 {
        (scale * val as f32) / f32::from(i16::MAX)
    }
}
