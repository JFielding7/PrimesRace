#include <stdio.h>
#include <malloc.h>
#include <time.h>
#include <stdbool.h>
#include <string.h>

unsigned long max(unsigned long a, unsigned long b) {
    return a > b ? a : b;
}

unsigned long get_starting_multiple(unsigned long curr_prime, unsigned long lower) {
    return max(curr_prime * curr_prime, curr_prime * ((lower - 1) / curr_prime + 1));
}

unsigned long *first_n_primes(unsigned long n) {
    unsigned long *primes = malloc(sizeof(unsigned long) * n);
    if (n > 0) primes[0] = 2;
    if (n > 1) primes[1] = 3;

    size_t primes_count = 2;
    unsigned long prime_factor_limit = 4;
    unsigned long lower = 5;
    unsigned long upper = 10;
    unsigned long inc = 7;

    while (primes_count < n) {
        size_t range = upper - lower;
        bool prime[range];
        memset(prime, true, range);

        // cross of composite numbers with prime factors less than the prime factor limit
        unsigned long curr_prime = primes[0];
        for (unsigned long i = 0; i < primes_count && curr_prime < prime_factor_limit; curr_prime = primes[++i]) {
            for (unsigned long curr_multiple = get_starting_multiple(curr_prime, lower); curr_multiple < upper; curr_multiple += curr_prime) {
                prime[curr_multiple - lower] = false;
            }
        }

        // append the next prime numbers
        for (unsigned long i = 0; i < range; i++) {
            if (prime[i]) {
                primes[primes_count++] = i + lower;
                if (primes_count == n) return primes;
            }
        }

        // update bounds
        lower = upper;
        upper += inc;
        inc += 2;
        prime_factor_limit++;
    }

    return primes;
}

int main() {
    const unsigned long n = 100000000;

    clock_t begin = clock();
    unsigned long *primes = first_n_primes(n);
    clock_t end = clock();

    printf("%luth prime: %ld\n", n, primes[n - 1]);
    double time_spent = (double)(end - begin) / CLOCKS_PER_SEC;
    printf("\nTime: %f\n", time_spent);

    free(primes);

    return 0;
}
