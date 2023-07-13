pub fn prime_sieve(n: usize) -> Vec<usize> {
    let mut is_prime: Vec<bool> = vec![true; n + 1];

    for i in is_prime.iter_mut().take(n + 1).skip(2) {
        *i = true;
    }

    let mut p: usize = 2;

    while p * p <= n {
        if is_prime[p] {
            let mut i: usize = p * p;

            while i <= n {
                is_prime[i] = false;
                i += p;
            }
        }

        p += 1;
    }

    is_prime[0] = false;
    is_prime[1] = false;

    is_prime.iter().enumerate().filter(|(_, is_prime)| **is_prime ).map(|(number, _)| number).collect()
}