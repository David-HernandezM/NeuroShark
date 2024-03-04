pub fn absolute(mut x: i128) -> i128 {
    if x < 0 {
        x *= -1;
        x
    } else {
        x
    }
}

pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    while b != 0 {
        a %= b;

        if a == 0 {
            return b;
        }

        b %= a;
    }

    a
}

