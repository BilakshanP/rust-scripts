const BASE_MAP: &[u8; 36]= b"0123456789abcdefghijklmnopqrstuvwxyz";
const BASE_MAP_EXTENDED: &[u8; 64]= b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";
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