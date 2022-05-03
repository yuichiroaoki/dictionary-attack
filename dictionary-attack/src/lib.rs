/// Generates a string Vec. For use with tests.
pub fn generate_string(i: u64) -> Vec<u8> {
    let mut array = [0u8; 20];
    generate_char_array(i, &mut array).to_vec()
}

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
const CHAR_COUNT: usize = 35;

fn generate_charactor(i: u8) -> char {
    // if i > 0x7f {
    //     panic!("generate_charactor: i > 0x7f");
    // }
    // let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    CHARSET.chars().nth(i as usize).unwrap()
}

#[test]
fn test_generate_charactor() {
    assert_eq!('a', generate_charactor(0));
    assert_eq!('z', generate_charactor(25));
    assert_eq!('0', generate_charactor(26));
    assert_eq!('9', generate_charactor(CHAR_COUNT as u8));
}

// pub fn number_to_string(i: u64) -> String {
// }


/// Generates a string slice (in u8) to crack based on an index seed.
/// An array needs to be passed to avoid performance penalties with allocating a Vec.
fn generate_char_array(mut i: u64, reversed: &mut [u8; 20]) -> &[u8] {
    if i == 0 {
        return &[];
    }
    let radix = 26;
    const A_DEC: u8 = 97;

    let mut digit = 0;
    while i > 0 {
        let remainder = i % radix;
        let remainder_zero_shifted = if remainder == 0 { radix } else { remainder };
        let m = remainder_zero_shifted - 1;

        i = (i - remainder_zero_shifted) / radix;

        reversed[digit] = A_DEC + m as u8;

        digit += 1;
    }
    for i in 0..(digit / 2) {
        let swap = reversed[digit - i - 1];
        reversed[digit - i - 1] = reversed[i];
        reversed[i] = swap;
    }
    &(reversed[0..digit])
}

#[test]
fn test_calculate_str_len() {
    assert_eq!(0, generate_string(0).len());
    assert_eq!(1, generate_string(1).len());
    assert_eq!(1, generate_string(26).len());
    assert_eq!(2, generate_string(27).len());
    assert_eq!(2, generate_string(702).len());
    assert_eq!(3, generate_string(703).len());
}


#[test]
fn test_generate_string() {
    assert_eq!("", String::from_utf8(generate_string(0)).unwrap());
    assert_eq!("a", String::from_utf8(generate_string(1)).unwrap());
    assert_eq!("z", String::from_utf8(generate_string(26)).unwrap());
    assert_eq!("aa", String::from_utf8(generate_string(27)).unwrap());
    assert_eq!("zz", String::from_utf8(generate_string(702)).unwrap());
    assert_eq!("aaa", String::from_utf8(generate_string(703)).unwrap());
    assert_eq!("zzz", String::from_utf8(generate_string(18278)).unwrap());
    assert_eq!("zzzz", String::from_utf8(generate_string(475254)).unwrap());
    assert_eq!("zzzzz", String::from_utf8(generate_string(12356630)).unwrap());
}
