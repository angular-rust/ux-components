use std::char;

/// A simple set of static helper functions for mint controls to use
pub struct Helper {}

impl Helper {
    // static
    pub fn sign(x: f32) -> i32 {
        if x < 0.0 {
            -1
        } else if x > 0.0 {
            1
        } else {
            0
        }
    }

    // static
    pub fn in_rect(x: f32, y: f32, rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
        if x < rx {
            return false;
        }

        if y < ry {
            return false;
        }

        if x > rx + rw {
            return false;
        }

        if y > ry + rh {
            return false;
        }

        true
    }

    // static
    // ?val:Option<i32>
    pub fn uniqueid(val: Option<u32>) -> String {
        let val = val.unwrap_or_else(rand::random);

        fn to_char(value: u32) -> char {
            if value > 9 {
                let mut ascii = 65 + (value - 10);
                if ascii > 90 {
                    ascii += 6;
                }
                char::from_u32(ascii).unwrap()
            } else {
                format!("{}", value).chars().next().unwrap()
            }
        } //to_char

        let r = (val % 62) as u32;
        let q = (val / 62) as u32;
        if q > 0 {
            return format!("{}{}", Self::uniqueid(Some(q)), to_char(r));
        } else {
            return format!("{}", to_char(r));
        }
    }
}
