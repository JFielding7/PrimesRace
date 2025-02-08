use std::cmp::max;
use std::time::Instant;

fn get_first_multiple(prime: usize, lower: usize) -> usize {
    max(prime * prime, prime * ((lower - 1) / prime + 1))
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
        let mut is_prime = vec![true; range];

        let mut i = 0;
        while i < primes.len() && primes[i] < prime_factor_limit {
            let prime = primes[i];
            let first_multiple = get_first_multiple(prime, lower);
            for multiple in (first_multiple..upper).step_by(prime) {
                is_prime[multiple - lower] = false;
            }
            i += 1;
        }

        for (i, &prime) in is_prime.iter().enumerate() {
            if prime {
                primes.push(i + lower);
                if primes.len() == n {
                    return primes;
                }
            }
        }

        prime_factor_limit += 1;
        range += 2;
        lower = upper;
        upper += range;
    }
}

fn main() {
    const n: usize = 100000000;

    let start = Instant::now();
    let primes = first_n_primes(n);
    let duration = start.elapsed();

    println!("{n}th Prime: {:?}", primes[n-1]);
    println!("Time: {:?}", duration);
}
