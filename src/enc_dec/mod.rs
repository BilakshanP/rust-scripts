#[allow(dead_code)]
#[allow(clippy::needless_doctest_main)]

/// ```
/// fn main() {
///     use functions::miscs::{string_to_vec, vec_to_string};
/// 
///     let mut primes: HashSet<usize> = HashSet::new();
///     let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
/// 
///     let mut n: usize = 0;
///     let mut public_key: usize = 0;
///     let mut private_key: usize = 0;
/// 
///     fill_primes(&mut primes);
///     set_keys(&mut n, &mut public_key, &mut private_key, &mut primes, &mut rng);
/// 
///     let original_message: &str = "Hello";
/// 
///     let message: Vec<usize> = string_to_vec(original_message).iter().map(|x: _| *x as usize).collect();
/// 
///     let encoded: Vec<usize> = encoder(&message, &public_key, &n);
///     let decoded: Vec<usize> = decoder(&encoded, &private_key, &n);
/// 
///     let decrypted_message: &str = &vec_to_string(&decoded.iter().map(|x: _| *x as u8).collect::<Vec<u8>>());
/// 
///     println!("pvk: {:?}\npbk: {:?}", (private_key, n), (public_key, n));
/// 
///     println!("Original message:  {}", original_message);
///     println!("As message:        {:?}", message);
///     println!("Encoded message:   {:?}", encoded);
///     println!("Decoded message:   {:?}", decoded);
///     println!("Decrypted message: {:?}", decrypted_message);
///     println!("Are equal:         {} & {}", original_message == decrypted_message, message == decoded)
/// }
/// ```
pub mod rsa;