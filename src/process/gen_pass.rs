use anyhow::Result;
use rand::prelude::SliceRandom;

const UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: u8,
    lowercase: u8,
    number: u8,
    symbol: u8,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase == 1 {
        chars.extend_from_slice(UPPERCASE);
        password.push(*UPPERCASE.choose(&mut rng).expect("won't be empty"))
    }
    if lowercase == 1 {
        chars.extend_from_slice(LOWERCASE);
        password.push(*LOWERCASE.choose(&mut rng).expect("won't be empty"))
    }
    if number == 1 {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("won't be empty"))
    }
    if symbol == 1 {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("won't be empty"))
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("won't be empty");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password_str = String::from_utf8(password)?;

    Ok(password_str)
}
