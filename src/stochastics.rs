pub fn faculty(n: i32) -> i32 {
    if n == 0 {
        1
    } else if n == 1 {
        n
    } else {
        n * faculty(n - 1)
    }
}

pub fn permutation(n: i32, karr: Vec<i32>) -> i32 {
    let sum: i32 = karr.iter().sum();
    if  sum != n {
        panic!("Sum of parts is not equal to whole");
    }

    let mut divisor = 1;

    for k in karr {
        divisor *= faculty(k);
    }

    faculty(n) / divisor
}

pub fn combination(n: i32, k: i32) -> i32 {
    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    faculty(n) / faculty(n - k) / faculty(k)
}

pub fn combination_with_repetition(n: i32, k: i32) -> i32 {
    let m = n + k - 1;
    faculty(m) / faculty(m - k) / faculty(k)
}

pub fn variation(n: i32, k: i32) -> i32 {
    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    faculty(n) / faculty(n - k)
}

pub fn variation_with_repetition(n: i32, k: i32) -> i32 {
    n.pow(k as u32)
}

pub fn binomial(n: i32, p: f64) -> Vec<f64> {
    let q = 1.0 - p;
    let mut binomial: Vec<f64> = Vec::with_capacity((n + 1) as usize);

    for x in 0..n+1 {
        let tmp = (faculty(n) as f64) / ((faculty(x) * faculty(n - x)) as f64);
        let result = tmp * p.powi(x) * q.powi(n - x);
        binomial.push(result);
    }

    binomial
}

