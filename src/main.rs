use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};

fn prime_sieve<W>(max_number_to_check: usize, mut output: W) -> u32
    where W: Write
{
    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;

    let mut total_primes_found = 0;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            writeln!(output, "{}", p).expect("Unable to write to file");
            total_primes_found += 1;
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }

    total_primes_found
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let n: usize = args[1].trim().parse().expect("Wanted a number");

    let ofile = &args[2];
    let f = File::create(ofile).expect("Unable to create file");
    let f = BufWriter::new(f);

    let np = prime_sieve(n, f);

    println!("Found {} primes less than {}. Wrote to {}", np, n, ofile);
}





// fn main() {
//     println!("start");
//     let mut prime = [true; 30000];

//     let mut p = 2;
//     while p * p < 30000 {
//         if prime[p] {
//             let mut i = p * p;
//             while i < 30000 {
//                 prime[i] = false;
//                 i += p
//             }
//         }
//         p += 1;
//     }
//     let mut index = 0;
//     for item in prime.iter() {
//         println!("{}:{}", index, item);
//         index += 1;
//     }
// }
