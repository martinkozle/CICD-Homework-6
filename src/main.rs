use std::{time::Instant};
mod prime_sieve;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about="Prime sieve")]
struct Args {
    #[clap(short='l', long="limit", default_value="100", help="The maximum number to check for primes")]
    limit: usize
}

fn main() {
    let args = Args::parse();
    let now = Instant::now();
    let primes = prime_sieve::prime_sieve(args.limit);
    let elapsed = now.elapsed();
    println!("Checked {} numbers and found {} primes in {:.3?} seconds.", args.limit, primes.len(), elapsed);
}
