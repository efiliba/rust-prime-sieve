fn main() {
  let sieve_size = usize::pow(10, 3);
  let mut sieve = vec![true; sieve_size + 1]; // Add extra 1 as 0 ignored (positions used as the number)

  run_sieve(&mut sieve);
  print_results(sieve, false);
}

// Number of primes by 10 to the power of index. e.g. 1000 = 10^4 => value at index 4 is 168 (168 prime numbers less than 1000)
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

fn print_results(sieve: Vec<bool>, show_results: bool) {
  if show_results {
    print!("2, ");
  }

  let mut count = 1;
  for i in (3..sieve.len()).step_by(2) {
    if sieve[i] {
      if show_results {
          print!("{:?}, ", i);
      }
      count += 1;
    }
  }

  println!("\nresult: {:?}", count);
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
  for i in 1..7 {
    let sieve_size = usize::pow(10, i);
    let mut sieve = vec![true; sieve_size + 1];
  
    run_sieve(&mut sieve);
    assert_eq!(count_set_sieve(sieve), PRIME_COUNTS[i as usize]);
  }
}
