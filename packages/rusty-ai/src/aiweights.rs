use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use common::misc::GenericErr;
use core::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::io::Cursor;

pub const NUM_AI_WEIGHTS: i32 = 34;
#[derive(Clone)]
pub struct AIWeights {
    /// 0: Perfect Clear\
    /// 1-4: 1-4 line clear\
    /// 5-14: Column num holes\
    /// 15-24: Column height\
    /// 25-33: Column deltas (difference between columns)
    pub values: [f32; NUM_AI_WEIGHTS as usize],
}
impl AIWeights {
    pub fn new() -> Self {
        AIWeights {
            values: [0.0; NUM_AI_WEIGHTS as usize],
        }
    }

    pub fn normalized(&self) -> Self {
        let mut mag = 0.0;
        for i in 0..NUM_AI_WEIGHTS {
            mag += self.values[i as usize].powf(2.0);
        }
        mag = mag.sqrt();
        // Prevent division by zero errors
        mag = if mag == 0.0 { 1.0 } else { mag };

        let mut values = self.values.clone();
        for i in 0..NUM_AI_WEIGHTS {
            values[i as usize] /= mag;
        }
        AIWeights { values }
    }
    pub fn cross_over(&self, other: &Self, self_weight: f32, other_weight: f32) -> Self {
        let mut values = [0.0; NUM_AI_WEIGHTS as usize];
        for i in 0..NUM_AI_WEIGHTS {
            values[i as usize] =
                self.values[i as usize] * self_weight + other.values[i as usize] * other_weight
        }
        AIWeights { values }.normalized()
    }
    pub fn mutate(&self, property: i32, amount: f32) -> Self {
        assert!(property >= 0 && property < NUM_AI_WEIGHTS);
        let mut values = self.values;
        values[property as usize] += amount;
        AIWeights { values }.normalized()
    }

    pub fn from_string(text: &str) -> Result<Self, GenericErr> {
        let bytes = match base65536::decode(text, false) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Error decoding weight string".into()),
        };
        let mut cursor = Cursor::new(bytes);
        let mut values = [0.0; NUM_AI_WEIGHTS as usize];
        for i in 0..NUM_AI_WEIGHTS {
            values[i as usize] = match cursor.read_f32::<BigEndian>() {
                Ok(val) => val,
                Err(_) => return Err("Error decoding weight string".into()),
            };
        }
        Ok(AIWeights { values })
    }
    pub fn to_string(&self) -> String {
        let mut bytes = Vec::new();
        for num in &self.values {
            bytes.write_f32::<BigEndian>(*num).unwrap();
        }
        base65536::encode(&bytes, None)
    }
}
impl TryFrom<String> for AIWeights {
    type Error = GenericErr;
    fn try_from(text: String) -> Result<Self, Self::Error> {
        AIWeights::from_string(&text)
    }
}
impl TryFrom<&str> for AIWeights {
    type Error = GenericErr;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        AIWeights::from_string(text)
    }
}
impl From<AIWeights> for String {
    fn from(weights: AIWeights) -> Self {
        weights.to_string()
    }
}
impl Display for AIWeights {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
