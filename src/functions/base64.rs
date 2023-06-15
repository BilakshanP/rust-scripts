const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/',
];

const BASE64_INDEX: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
    b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
    b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
    b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

///
/// Derieved using:
/// ```
/// let mut values: Vec<isize> = vec![-1; 256];
/// 
/// for i in 0..64 {
///     values[BASE64_CHARS[i] as usize] = i as isize
/// }
/// 
/// println!("{}", values);
/// ```
const VALID_VALUES: [isize; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
    -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

const BASE_MAP: &[u8; 36]= b"0123456789abcdefghijklmnopqrstuvwxyz";
pub const BASE_MAP_EXTENDED: &[u8; 64]= b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";

/// ```
/// let mut new = vec![-1; 128];
///
/// for (n, i) in b64::BASE_MAP_EXTENDED.iter().enumerate() {
///    new[*i as usize] = n as i32
/// } 
/// println!("{:?}", new) 
/// ```
const BASE_MAP_EXTENDED_INDEX: [i32; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, -1, -1, -1, -1, -1, -1,
    -1, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1,
    -1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1
];

const RADIX_CHARS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', '+', '/'
];


/// vec to vec
pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    let mut val: usize = 0;
    let mut bits: i32 = -6;
    let  mask: usize = 0x3F;

    for c in input {
        val = (val << 8) + *c as usize;
        bits += 8;
    
        while bits >= 0 {
            encoded.push(BASE64_INDEX[(val >> bits) & mask]);
            bits -= 6;
        }
    }

    if bits > -6 {
        encoded.push(BASE64_INDEX[((val << 8) >> (bits + 8)) & mask]);
    }

    while encoded.len() % 4 != 0 {
        encoded.push(b'=')
    }

    encoded
}

/// vec to vec
pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();

    let mut val: isize = 0;
    let mut bits: i32 = -8;

    for c in input {
        if VALID_VALUES[*c as usize] == -1 {
            if c == &b'=' {
                break;
            }

            panic!("{} is an invalid base64 literal", c)
        }

        val = (val << 6) + VALID_VALUES[*c as usize];
        bits += 6;

        if bits >= 0 {
            decoded.push(((val >> bits) & 0xFF) as u8);
            bits -= 8;
        }
    }

    decoded
}

pub fn str2str_encode(input: &str) -> String {
    let mut encoded: String = String::new();
    let mut val: usize = 0;
    let mut bits: i32 = -6;
    let  mask: usize = 0x3F;

    for c in input.chars() {
        val = (val << 8) + c as usize;
        bits += 8;
    
        while bits >= 0 {
            encoded.push(BASE64_CHARS[(val >> bits) & mask]);
            bits -= 6;
        }
    }

    if bits > -6 {
        encoded.push(BASE64_CHARS[((val << 8) >> (bits + 8)) & mask]);
    }

    while encoded.len() % 4 != 0 {
        encoded.push('=')
    }

    encoded
}

pub fn str2str_decode(input: &str) -> String {
    let mut decoded: String = String::new();
    let mut values: Vec<isize> = vec![-1; 256];

    for i in 0..64 {
        values[BASE64_CHARS[i] as usize] = i as isize
    }

    let mut val: isize = 0;
    let mut bits: i32 = -8;

    for c in input.chars() {
        if values[c as usize] == -1 {
            break; // invalid char detected
        }

        val = (val << 6) + values[c as usize];
        bits += 6;

        if bits >= 0 {
            decoded.push(((val >> bits) & 0xFF) as u8 as char);
            bits -= 8;
        }
    }

    decoded
}

pub fn vec2str_encode(input: &[u8]) -> String {
    let mut encoded: String = String::new();
    let mut val: usize = 0;
    let mut bits: i32 = -6;
    let  mask: usize = 0x3F;

    for c in input {
        val = (val << 8) + *c as usize;
        bits += 8;
    
        while bits >= 0 {
            encoded.push(BASE64_INDEX[(val >> bits) & mask] as char);
            bits -= 6;
        }
    }

    if bits > -6 {
        encoded.push(BASE64_INDEX[((val << 8) >> (bits + 8)) & mask] as char);
    }

    while encoded.len() % 4 != 0 {
        encoded.push('=')
    }

    encoded
}

pub fn vec2str_decode(input: &[u8]) -> String {
    let mut decoded: String = String::new();

    let mut val: isize = 0;
    let mut bits: i32 = -8;

    for c in input {
        if VALID_VALUES[*c as usize] == -1 {
            if c == &b'=' {
                break;
            }

            panic!("{} is an invalid base64 literal", c)
        }

        val = (val << 6) + VALID_VALUES[*c as usize];
        bits += 6;

        if bits >= 0 {
            decoded.push(((val >> bits) & 0xFF) as u8 as char);
            bits -= 8;
        }
    }

    decoded
}

pub fn str2vec_encode(input: &str) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    let mut val: usize = 0;
    let mut bits: i32 = -6;
    let  mask: usize = 0x3F;

    for c in input.chars() {
        val = (val << 8) + c as usize;
        bits += 8;
    
        while bits >= 0 {
            encoded.push(BASE64_INDEX[(val >> bits) & mask]);
            bits -= 6;
        }
    }

    if bits > -6 {
        encoded.push(BASE64_INDEX[((val << 8) >> (bits + 8)) & mask]);
    }

    while encoded.len() % 4 != 0 {
        encoded.push(b'=')
    }

    encoded
}

/// ```
/// todo!()
/// ```
pub fn str2vec_decode(input: &str) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();

    let mut val: isize = 0;
    let mut bits: i32 = -8;

    for c in input.chars() {
        if VALID_VALUES[c as usize] == -1 {
            if c == '=' {
                break;
            }

            panic!("{} is an invalid base64 literal", c)
        }

        val = (val << 6) + VALID_VALUES[c as usize];
        bits += 6;

        if bits >= 0 {
            decoded.push(((val >> bits) & 0xFF) as u8);
            bits -= 8;
        }
    }

    decoded
}


/// not just b64, custom implementation
pub fn from_str_radix(number: u128, radix: u8) -> String {
    let radix: u128 = radix as u128;

    if number == 0 {
        return String::from("0");
    }
    if radix > 36 {
        panic!("Can't have a radix greater than 36: {}", radix)
    }
    
    let mut digits: Vec<char> = Vec::new();
    let mut num: u128 = number;
    while num > 0 {
        let remainder = num % radix;
        digits.push(BASE_MAP[remainder as usize] as char);
        num /= radix;
    }
    
    digits.reverse();

    let mut out_str = String::with_capacity(digits.len());

    for digit in digits {
        out_str.push(digit)
    }

    out_str
}

// custom implementation
pub fn from_str_radix_extended(number: u128, radix: u8) -> String {
    let radix: u128 = radix as u128;

    if number == 0 {
        return String::from("0");
    }
    if radix > 64 {
        panic!("Can't have a radix greater than 36: {}", radix)
    }
    
    let mut digits: Vec<char> = Vec::new();
    let mut num: u128 = number;
    while num > 0 {
        let remainder = num % radix;
        digits.push(BASE_MAP_EXTENDED[remainder as usize] as char);
        num /= radix;
    }
    
    digits.reverse();

    let mut out_str = String::with_capacity(digits.len());

    for digit in digits {
        out_str.push(digit)
    }

    out_str
}

/// can overflow, custom implementation
pub fn to_decimal(number: &str, radix: u32) -> u128 {
    if radix > 64 {
        panic!("Radix greater that 64 is unsipported, provided radix: {}", radix)
    }

    let mut decimal = 0;
    let mut power = 1;

    for ch in number.chars().rev() {
        if let Some(index) = RADIX_CHARS.iter().position(|&c| c == ch) {
            if index > radix as usize {
                panic!("Out of range digit {} for radix {}", ch, radix) 
            }
            decimal += index as u128 * power;
            power *= radix as u128;
        } else if ch != '_' {
            panic!("Out of radix digit: {}", ch)          
        }
    }

    decimal
}