use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Pow;

pub fn miller_rabin_biguint(n: &BigUint) -> bool {
    // Create variables to store 0, 1, and 2 as `BigUint`s. We will use these
    // values at various times in the function, so let's just allocate them
    // once and reference them as needed.
    let zero = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);

    // Do some preliminary checks.
    if *n == zero || *n == one {
        return false;
    }
    if *n == two {
        return true;
    }
    if !n.is_odd() {
        return false;
    }

    // If `n` is odd, then `n - 1` is even. Every even positive integer can
    // be expressed as a multiple of a power-of-two: `m * 2^k`. First, we find
    // the largest power-of-two `2^k` that evenly divides `n - 1` by solving the
    // following equation for the largest `k`: `n - 1 = m * 2^k`.
    let minus_one: BigUint = n - 1u8;
    let mut k = 1u64;
    while &minus_one % two.pow(k) == zero {
        k += 1;
    }
    let m: BigUint = &minus_one / two.pow(k - 1);

    // Next we choose `a` s.t. `1 < a < n` and use it to calculate `b_0` where
    // `b_0 = a^m % n`. If `b_0` is 1 or -1 modulo `n`, then `n` is probably
    // prime.
    let a = &two;
    let b_0: BigUint = a.modpow(&m, n);
    if b_0 == one || b_0 == minus_one {
        return true;
    }

    // If we did not return in the above step, we compute `b_i` until it is
    // either 1 or -1 modulo `n`. If `b_i` is 1, then `n` is composite. If `b_i`
    // is -1 modulo `n`, then `n` is probably prime.
    let mut b_prev = b_0;
    loop {
        let b_i = b_prev.modpow(&two, n);
        if b_i == one {
            return false;
        } else if b_i == minus_one {
            return true;
        } else {
            b_prev = b_i;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_bigint::BigUint;

    use crate::miller_rabin_biguint;

    #[test]
    fn test_primality() {
        let one = BigUint::from(1u8);
        assert_eq!(miller_rabin_biguint(&one), false);
	    
        let two = BigUint::from(2u8);
        assert!(miller_rabin_biguint(&two));
        
        let fifty_two = BigUint::from(52u8);
        assert_eq!(miller_rabin_biguint(&fifty_two), false);
	    
        let fifty_three = BigUint::from(53u8);
        assert!(miller_rabin_biguint(&fifty_three));

        let big_64_bit_prime = BigUint::from(15930455692162817671u64);
        assert!(miller_rabin_biguint(&big_64_bit_prime));

        let big_255_bit_prime = BigUint::from_str("57896044618658097711785492504343953926634992332820282019728792003956564819949").unwrap();
        assert!(miller_rabin_biguint(&big_255_bit_prime));
    }
}
