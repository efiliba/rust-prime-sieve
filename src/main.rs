use std::time::{Instant, Duration};

fn main() {
  let sieve_size = usize::pow(10, 6); // Upper limit - One million
  let duration = Duration::new(5, 0); // Five seconds
  let mut passes = 0;
  let start = Instant::now();

  let mut sieve = Vec::new();
  while Instant::now() - start < duration {
    sieve = vec![true; sieve_size + 1]; // Add extra 1 as 0 ignored (positions used as the number)
    run_sieve(&mut sieve);
    passes += 1;
  }
  print_results(sieve, Instant::now() - start, passes, sieve_size, false);
}

// Number of primes by 10 to the power of index. e.g. 1 000 000 = 10^6 => value at index 6 is 78498
const PRIME_COUNTS: [usize; 9] = [0, 4, 25, 168, 1229, 9592, 78498, 664579, 5761455];

fn run_sieve(sieve: &mut Vec<bool>) {
  let mut factor: usize = 3;
  let sieve_size = sieve.len();
  let upper_limit = (sieve_size as f64).sqrt() as usize + 1;

  // println!("upper_limit: {:?}", upper_limit);

  while factor < upper_limit {
    // find next unmarked factor
    for i in (factor..sieve_size).step_by(2) {
      if sieve[i] {
        factor = i;
        break;
      }
    }

    // remove all factors - clear bits at factor positions
    for i in (factor * 3..sieve_size).step_by(factor * 2) {
      sieve[i] = false;
    }

    factor += 2;
  }
}

fn print_results(sieve: Vec<bool>, duration: Duration, passes: usize, sieve_size: usize, show_results: bool) {
  if show_results {
    print!("2, ");
  }

  let mut count = 1;
  for i in (3..sieve_size).step_by(2) {
    if sieve[i] {
      if show_results {
        print!("{:?}, ", i);
      }
      count += 1;
    }
  }

  if show_results {
    println!("");
  }

  let verified_message = if validate_results(sieve, sieve_size) { "verified" } else { "incorrect" };
  println!("{} passes in {:?}, upper limit: {}, result: {} ({})", passes, duration, sieve_size, count, verified_message);
}

fn validate_results(sieve: Vec<bool>, sieve_size: usize) -> bool {
  let power = f32::log(sieve_size as f32, 10.0) as usize;
  count_set_sieve(sieve) == PRIME_COUNTS[power]
}

fn count_set_sieve(sieve: Vec<bool>) -> usize {
  let mut count = 1;
  for i in (3..sieve.len()).step_by(2) {
    if sieve[i] {
      count += 1;
    }
  }
  count
}

#[test]
fn should_return_the_correct_number_of_primes_at_each_base_10() {
  for i in 1..9 {
    let sieve_size = usize::pow(10, i);
    let mut sieve = vec![true; sieve_size + 1];
  
    run_sieve(&mut sieve);
    assert!(validate_results(sieve, sieve_size));
  }
}
