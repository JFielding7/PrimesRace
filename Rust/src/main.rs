use std::cmp::max;
use std::time::Instant;

fn get_first_multiple(prime: usize, lower: usize) -> usize {
    max(prime * prime, prime * ((lower - 1) / prime + 1))
}

fn prime_sieve(primes: &Vec<usize>, prime_factor_limit: usize, lower: usize, upper: usize) -> Vec<bool> {
    let mut is_prime = vec![true; upper - lower];

    let mut i = 0;
    while i < primes.len() && primes[i] < prime_factor_limit {
        for multiple in (get_first_multiple(primes[i], lower)..upper).step_by(primes[i]) {
            is_prime[multiple - lower] = false;
        }
        i += 1;
    }

    is_prime
}

fn append_primes(primes: &mut Vec<usize>, is_prime: &Vec<bool>, lower: usize, n: usize) -> bool {
    for (i, &prime) in is_prime.iter().enumerate() {
        if prime {
            primes.push(i + lower);
            if primes.len() == n {
                return true;
            }
        }
    }

    false
}

fn first_n_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::with_capacity(n);
    if n > 0 {
        primes.push(2);
    }
    if n > 1 {
        primes.push(3);
    }

    let mut prime_factor_limit = 4;
    let mut range = 5;
    let mut lower = 5;
    let mut upper = 10;

    loop {
        let is_prime = prime_sieve(&primes, prime_factor_limit, lower, upper);
        if append_primes(&mut primes, &is_prime, lower, n) {
            return primes;
        }

        prime_factor_limit += 1;
        range += 2;
        lower = upper;
        upper += range;
    }
}

fn main() {
    const N: usize = 100000000;

    let start = Instant::now();
    let primes = first_n_primes(N);
    let duration = start.elapsed();

    println!("{N}th Prime: {:?}", primes[N -1]);
    println!("Time: {:?}", duration);
}
