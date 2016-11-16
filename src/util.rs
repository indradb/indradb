use rand::{Rng, OsRng};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn generate_random_secret() -> String {
    let mut chars = vec![];
	let options = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = OsRng::new().unwrap();

    for _ in 0..32 {
		let c: u8 = *rng.choose(options).unwrap();
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}

pub fn get_salted_hash(salt: &str, pepper: Option<&str>, secret: &str) -> String {
	let mut sha = Sha256::new();
	sha.input(salt.as_bytes());

	if pepper.is_some() {
		sha.input(pepper.unwrap().as_bytes());
	}

	sha.input(secret.as_bytes());
	return format!("1:{}", sha.result_str());
}
