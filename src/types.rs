use std::fmt;
use std::error::Error;



pub type MQTTByte = u8; 
pub type MQTTTwoBytes = u16; 
pub struct MQTTString {
    len: MQTTTwoBytes,
    data: String
}

impl MQTTString {
    pub fn from(s: String) -> Option<Self> {
        if s.len() <= u16::MAX as usize {
            Some(MQTTString{len: s.len() as u16, data: s})
        } else {
            None
        }

    }
}

pub struct VarByte {
    data: Vec<u8>
}

#[derive(Debug)]
pub struct ParseVarByteError;

impl fmt::Display for ParseVarByteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse into VarByte")
    }
}

impl Error for ParseVarByteError {}

impl From<u32> for VarByte {
    fn from(value: u32) -> Self {
        let mut x = value;
        let mut v: Vec<u8> = vec![];

        loop {

            let mut b: u8 = (x % 128).try_into().unwrap();
            x /= 128;
            if x > 0 {
                b |= 0b10000000;
            }
            v.push(b);
            if x == 0 { break; }
        }

        VarByte { data: v }
    }
}

impl From<VarByte> for u32 {
    fn from(value: VarByte) -> Self {
        let mut multiplier = 1;
        let mut v: u32 = 0;
        let mut b: u32;
        let mut i = 0;

        loop {
            b = value.data[i] as u32;
            v += (b & 127) * multiplier;
            if multiplier > 128*128*128 {
                // TODO: Error handling?
            }
            multiplier *= 128;
            i += 1;
            if (b & 128) == 0 { break; }
        }

        v
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_string() {
        let s = MQTTString::from(String::from("Hello world")).unwrap();
        assert_eq!(s.len, 11u16);
        assert_eq!(s.data, String::from("Hello world"));
    }

    #[test]
    fn create_invalid_string() {
        let single_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
        let long_string = single_string.repeat(100000);

        let s = MQTTString::from(long_string);
        assert!(s.is_none());
    }

    #[test]
    fn single_byte_var_byte() {
        let v = VarByte::from(127);
        assert_eq!(1, v.data.len());
        assert_eq!(127, v.data[0]);
    }

    #[test]
    fn two_bytes_var_byte() {
        let v = VarByte::from(128);
        assert_eq!(2, v.data.len());
        assert_eq!(128, v.data[0]);
        assert_eq!(1, v.data[1]);
    }

    #[test]
    fn four_bytes_var_byte() {
        let v = VarByte::from(268_435_455);
        assert_eq!(4, v.data.len());
        assert_eq!(255, v.data[0]);
        assert_eq!(255, v.data[1]);
        assert_eq!(255, v.data[2]);
        assert_eq!(127, v.data[3]);
    }

    #[test]
    fn u32_from_varbyte() {
        let u = u32::from(VarByte::from(1234567));
        assert_eq!(1234567u32, u)
    }
}
