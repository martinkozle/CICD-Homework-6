use bit_vec::BitVec;
use rayon::prelude::*;

pub fn prime_sieve(max_n: usize) -> Vec<i64> {
    let mut sieve = BitVec::from_elem(max_n / 2 + 2, true);
    let to: usize = (max_n as f32).sqrt() as usize + 1;
    for i in (3..to).step_by(2) {
        if sieve[i >> 1] {
            for j in (i * i..max_n + 1).step_by(2 * i) {
                sieve.set(j >> 1, false);
            }
        }
    }
    let mut primes: Vec<i64> = vec![2];
    primes.par_extend(
        (3..max_n + 1)
            .into_par_iter()
            .step_by(2)
            .filter(|i| sieve[i >> 1])
            .map(|i| i as i64),
    );
    primes
}
