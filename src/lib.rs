pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn gcd(mut m: i32, mut n: i32) -> i32 {
    let mut m_prime;
    loop {
        m_prime = m % n;
        if m_prime == 0 {
            break;
        }
        m = n;
        n = m_prime;
    }
    n
}

pub fn lcm(m: i32, n: i32) -> i32 {
    let g = gcd(m, n);
    m * n / g
}

pub fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut p: u64 = 0;
    let mut c: u64 = 1;
    for _ in 0..n - 1 {
        let t: u64 = c;
        c = p + c;
        p = t;
    }
    c
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    #[test]
    fn test_fib_0() {
        assert_eq!(fibonacci(0u64), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fibonacci(1u64), 1);
    }

    #[test]
    fn test_fib_10() {
        assert_eq!(fibonacci(10u64), 55);
    }

    #[test]
    fn test_fib_43() {
        assert_eq!(fibonacci(43u64), 433494437);
    }

    #[test]
    fn test_lcm_12_20() {
        assert_eq!(lcm(12, 20), 60);
    }

    #[test]
    fn test_lcm_100_201() {
        assert_eq!(lcm(100, 201), 20100);
    }

    #[test]
    fn test_gcd_f42() {
        assert_eq!(gcd(267914296, 29), 29);
    }

    #[test]
    fn test_gcd_60_45() {
        assert_eq!(gcd(60, 45), 15);
    }

    fn comp_vec(u: Vec<i32>, v: Vec<i32>) -> bool {
        if u.len() != v.len() {
            return false;
        }
        for i in 0..u.len() {
            if u[i] != v[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_in_order_bubble_sort() {
        let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        bubble_sort(&mut x);
        let are_equal = comp_vec(t, x);
        assert_eq!(true, are_equal);
    }
    #[test]
    fn test_reverse_order_bubble_sort() {
        let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut x = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut x);
        let are_equal = comp_vec(t, x);
        assert_eq!(true, are_equal);
    }
    #[test]
    fn test_random_order_bubble_sort() {
        let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut x = vec![3, 8, 2, 6, 9, 7, 4, 1, 5];
        bubble_sort(&mut x);
        let are_equal = comp_vec(t, x);
        assert_eq!(true, are_equal);
    }
    #[test]
    fn test_not_equal_bubble_sort() {
        let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut x = vec![3, 8, 2, 6, 5, 7, 4, 1, 5];
        bubble_sort(&mut x);
        let are_equal = comp_vec(t, x);
        assert_ne!(true, are_equal);
    }
}
