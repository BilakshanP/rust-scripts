use num::Integer;
use rand::Rng;

use std::collections::HashSet;

fn fill_primes(primes: &mut HashSet<usize>) {
    let mut seive: Vec<bool> = vec![true; 250];
    seive[0] = false;
    seive[1] = false;
    
    for i in 2..250 {
        for j in ((i * 2)..250).step_by(i) {
            seive[j] = false
        }
    }

    for (value, check) in seive.iter().enumerate() {
        if *check {
            primes.insert(value);
        }
    }
}

fn pick_random(primes: &mut HashSet<usize>, rng: &mut rand::rngs::ThreadRng) -> usize {
    let x = rng.gen_range(0..primes.len());

    let mut it = primes.iter().cloned();

    for _ in 0..x {
        it.next();
    }

    let ret = it.next().expect("No primes to choose.");
    primes.remove(&ret);

    ret
}

fn set_keys(n: &mut usize, public_key: &mut usize, private_key: &mut usize, primes: &mut HashSet<usize>, rng: &mut rand::rngs::ThreadRng) {
    let p: usize = pick_random(primes, rng);
    let q: usize = pick_random(primes, rng);

    *n = p * q;

    let phi = (p - 1) * (q - 1);

    let mut e: usize = 2;
    loop {
        if e.gcd(&phi) == 1 {
            break;
        }

        e += 1
    }

    *public_key = e;

    let mut d: usize = 2;
    loop {
        if (d * e) % phi == 1 {
            break;
        }

        d += 1
    }

    *private_key = d;
}

fn encrypt(message_slice: usize, public_key: &usize, n: &usize) -> usize {
    let n: usize = *n;
    let mut e: usize = *public_key;
    let mut encrypted_message_slice: usize = 1;

    while e > 0 {
        encrypted_message_slice *= message_slice;
        encrypted_message_slice %= n;
        e -= 1;
    }

    encrypted_message_slice
}

fn decrypt(encrypted_message_slice: usize, private_key: &usize, n: &usize) -> usize {
    let n: usize = *n;
    let mut d: usize = *private_key;
    let mut decrypted_message_slice: usize = 1;

    while d > 0 {
        decrypted_message_slice *= encrypted_message_slice;
        decrypted_message_slice %= n;
        d -= 1;
    }

    decrypted_message_slice
}

fn encoder(message: &[usize], public_key: &usize, n: &usize) -> Vec<usize> {
    let mut encoded: Vec<usize> = Vec::with_capacity(message.len());

    for message_slice in message {
        encoded.push(
            encrypt(*message_slice, public_key, n)
        )
    }

    encoded
}

fn decoder(encrypted_message: &[usize], private_key: &usize, n: &usize) -> Vec<usize> {
    let mut decoded: Vec<usize> = Vec::with_capacity(encrypted_message.len());

    for encrypted_message_slice in encrypted_message {
        decoded.push(
            decrypt(*encrypted_message_slice, private_key, n)
        )
    }

    decoded
}