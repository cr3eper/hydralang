
pub fn gcd(a: i64, b: i64) -> i64{
    fn _gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { _gcd(b, a % b) }
    }
    let a = a.abs();
    let b = b.abs();

    if a > b {
        _gcd(a, b)
    } else {
        _gcd(b, a)
    }
} 