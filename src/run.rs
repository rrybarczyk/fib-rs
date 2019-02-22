// O(2^n)
fn fib_recursive(n: usize) -> usize {
    if n == 1 || n == 2 {
        1
    } else {
        fib_recursive(n-1) + fib_recursive(n-2)
    }

}

// (2n+1) * O(1) = O(2n+1) = O(n)
fn fib_memo(n: usize, arr: &mut Vec<usize>) -> usize {
    if arr[n] == 0 {
        if n==1 || n==2 {
            arr[n] = 1;
        } else {
            arr[n] = fib_memo(n-1, arr) + fib_memo(n-2, arr);
        }
    }
    arr[n]
}

fn fib_memoized(n: usize) -> usize {
    let mut arr = vec![0; n+1];
    fib_memo(n, &mut arr)
}

// O(n)
fn fib_bottom_up(n: usize) -> usize {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut bottom_up = vec![0; n];
    bottom_up[0] = 1;
    bottom_up[1] = 1;

    for i in 2..n {
        bottom_up[i] = bottom_up[i-1] + bottom_up[i-2];
    }
    bottom_up[n-1]
}

pub fn run() {
    let n = 6;
    println!("Recursive: {}", fib_recursive(n));
    println!("Memoized: {}", fib_memoized(n));
    println!("Bottom Up: {}", fib_bottom_up(n));
}

#[test]
fn fib_recursive_method() {
    assert_eq!(fib_recursive(3), 2);
    assert_eq!(fib_recursive(5), 5);
    assert_eq!(fib_recursive(6), 8);
}

#[test]
fn fib_memoized_method() {
    assert_eq!(fib_memoized(3), 2);
    assert_eq!(fib_memoized(5), 5);
    assert_eq!(fib_memoized(6), 8);
}
#[test]
fn fib_bottom_up_method() {
    assert_eq!(fib_bottom_up(3), 2);
    assert_eq!(fib_bottom_up(5), 5);
    assert_eq!(fib_bottom_up(6), 8);
}
