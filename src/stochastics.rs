use ::basic::upow;

pub fn faculty(n: u64) -> u64 {
    if n == 0 {
        1
    } else if n == 1 {
        n
    } else {
        n * faculty(n - 1)
    }
}

pub fn permutation(n: u64, karr: Vec<u64>) -> u64 {
    let sum: u64 = karr.iter().sum();
    if  sum != n {
        panic!("Sum of parts is not equal to whole");
    }

    let mut divisor = 1;

    for k in karr {
        divisor *= faculty(k);
    }

    faculty(n) / divisor
}

pub fn combination(n: u64, k: u64) -> u64 {
    if (k > n) {
        panic!("Number of selections outgrows the number of elements");
    }

    faculty(n) / faculty(n - k) / faculty(k)
}

pub fn combination_with_repetition(n: u64, k: u64) -> u64 {
    let m = n + k - 1;
    faculty(m) / faculty(m - k) / faculty(k)
}

pub fn variation(n: u64, k: u64) -> u64 {
    if (k > n) {
        panic!("Number of selections outgrows the number of elements");
    }

    faculty(n) / faculty(n - k)
}

pub fn variation_with_repetition(n: u64, k: u64) -> u64 {
    upow(n, k)
}