// Rust does not seem to have a built
// in integer_sqrt. This is likely fine
// for low numbers
fn integer_sqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

// Very naive implementation based on Pseudocode from
// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    if n <= 1 {
        panic!("n must be greater than 1")
    }

    let mut a = vec![true; n as usize];

    for i in 2..=integer_sqrt(n) {
        if a[i as usize] {
            let mut j = i * i;
            while j < n {
                a[j as usize] = false;
                j += i;
            }
        }
    }

    let mut ret: Vec<u64> = vec![];
    for i in 2..n {
        if a[i as usize] {
            ret.push(i);
        }
    }

    ret
}

fn main() {
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_square_roots() {
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(5), 2);
        assert_eq!(integer_sqrt(16), 4);
    }

    #[test]
    fn sieve() {
        assert_eq!(sieve_of_eratosthenes(10), vec![2,3,5,7]);
        assert_eq!(sieve_of_eratosthenes(100), vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97]);
    }
}