use gstd::prelude::*;

use crate::utils::*;

pub type FractionTuple = (i128, i128);

const MAX_NUM: i128 = 9999999999999;

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Fraction {
    pub num: i128,
    pub den: i128
}

// Fraction struct that represents a rational number with a numerator
// a numerator or a denominator.
impl Fraction {
    pub fn new(num: i128 ,den: i128) -> Fraction {
        let mut new_fraction = Fraction {
            num,
            den
        };
        
        if den != 1 && num != 1 {
            new_fraction.cancel();
        }

        new_fraction.round();
        new_fraction
    }

    pub fn new_from_int(num: i128) -> Fraction {
        Fraction {
            num,
            den: 1
        }
    }

    pub fn new_from_tuple(values: FractionTuple) -> Fraction {
        Fraction {
            num: values.0,
            den: values.1
        }
    }

    // Fraction structs are always canceled and the denominator
    // is never negative.
    pub fn cancel(&mut self) {
        let g = gcd(absolute(self.num), absolute(self.den)).max(1);

        if g <= 0 {
            return;
        }        
        self.num /= g;
        self.den /= g;

        if self.den < 0 {
            self.num = -1 * self.num;
            self.den = -1 * self.den;
        }

        self.round();
    }

    // Round number to only ten decimals
    pub fn round(&mut self) {
        let (mut num, sign) = if self.num < 0 { 
            (-1 * self.num, -1) 
        } else { 
            (self.num, 1) 
        };
    
        while num > MAX_NUM && self.den > MAX_NUM {
            num /= 10;
            self.den /= 10;
        }
    
        self.num = num * sign;
    }

    // add "summand" to fraction and return new struct
    pub fn add(&self, summand: &Fraction) -> Fraction {
        Fraction::new(
            self.num * summand.den + self.den * summand.num,
            self.den * summand.den
        )
    }
    
    // add "summand" to fraction and save result in itself
    pub fn add_self(&mut self, summand: &Fraction) {
        self.num = self.num * summand.den + self.den * summand.num;
        self.den = self.den * summand.den;
        self.cancel();
    }

    // subtract "subtrahend" from fraction and return new struct
    pub fn sub(&self, subtrahend: &Fraction) -> Fraction {
        Fraction::new(
            self.num * subtrahend.den - self.den * subtrahend.num,
            self.den * subtrahend.den
        )
    }

    // subtract "subtrahend" from fraction and set to itself
    pub fn sub_self(&mut self, subtrahend: &Fraction) {
        self.num = self.num * subtrahend.den - self.den * subtrahend.num;
        self.den = self.den * subtrahend.den;
        self.cancel();
    }

    // subtract integer k in-place
    pub fn sub_int(&mut self, k: i128) {
        self.num -= k * self.den;
        self.cancel();
    }

    // multiply fraction with "factor" and return new struct
    pub fn mult(&self, factor: &Fraction) -> Fraction {
        Fraction::new(
            self.num * factor.num,
            if !self.is_zero() && !factor.is_zero() {
                self.den * factor.den
            } else {
                1
            }
        )
    }

    // multiply fraction with "factor" and set to iteself
    pub fn mult_self(&mut self, factor: &Fraction) {
        if !self.is_zero() && !factor.is_zero() {
            self.num = self.num * factor.num;
            self.den = self.den * factor.den;
        } else {
            self.num = 0;
            self.den = 1;
        }
        
        self.cancel();
    }

    // multiply ffraction in-place with ten
    pub fn mult_ten(&mut self) {
        self.num *= 10;
        self.cancel();
    }

    // divide fraction by "divisor" ans return new struct
    pub fn div(&self, divisor: &Fraction) -> Fraction {
        assert!(!divisor.is_zero(), "cant divide by zero");
        
        Fraction::new(
            self.num * divisor.den,
            if !self.is_zero() {
                self.den * divisor.num
            } else {
                1
            }
        )
    }

    // divide fraction by "divisor" and set the value to itself
    pub fn div_self(&mut self, divisor: &Fraction) {
        assert!(!divisor.is_zero(), "cant divide by zero");
        
        if !self.is_zero() {
            self.num = self.num * divisor.den;
            self.den = self.den * divisor.num;
        } else {
            self.num = 0;
            self.den = 1;
        }
        
        self.cancel();
    }

    // whether the fraction and "other" are equal
    pub fn eq(&self, other: &Fraction) -> bool {
        self.num * other.den == self.den * other.num
    }

    // whether fraction is greater than "other"
    pub fn gt(&self, other: &Fraction) -> bool {
        if self.num > 0 && other.num <= 0 {
            return true;
        }

        if self.num <= 0 && other.num > 0 {
            return false;
        }

        self.num * other.den > self.den * other.num
    }

    // whether fraction is greater than "other" or equal to it
    pub fn ge(&self, other: &Fraction) -> bool {
        self.gt(other) || self.eq(other)
    }

    // whether fraction is less than "other"
    pub fn lt(&self, other: &Fraction) -> bool {
        self.gt(other)
    }

    // whether fraction is less than "other" or equal to it
    pub fn le(&self, other: &Fraction) -> bool {
        self.lt(other) || self.eq(other)
    }

    // whether fraction is positive
    pub fn is_positive(&self) -> bool {
        self.num > 0
    }

    // whether fraction is negative
    pub fn is_negative(&self) -> bool {
        self.num < 0
    }

    // whether the fraction is equal to zero
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    // return the absolute value of the fraction as a new struct
    pub fn abs(&self) -> Fraction {
        Fraction::new(
            absolute(self.num),
            self.den
        )
    }

    // Set the fraction to its absolute value
    pub fn self_abs(&mut self) {
        self.num = absolute(self.num);
    }

    // Computes the integer part of the fraction as i128
    // Assumes that the fraction is positive
    pub fn floor(&self) -> i128 {
        self.num / self.den
    }
    
    // simplify the fraction by two, if ti can by simplified
    pub fn simplify_by_two(&mut self) {
        if self.num & 1 != 0 {
            self.num /= 2;
        }
    }

    // helper function which computes the product of "val" and 10
    // to the "exp" power and returns it as a fraction
    pub fn fraction_times_power_10(val: i128, exp: i32) -> Fraction {
        if exp >= 0 {
            let pow = i128::pow(10, exp as u32);
            Fraction::new_from_int(
                if val != 1 { val * pow } else { pow }
            )
        } else {
            let pow = i128::pow(10, (-1 * exp) as u32);
            Fraction::new(
                val, 
                pow
            )
        }
    }

    // helper function which computes the product of "val" and 
    // 2 to the "exp" power and return it as a fraction
    pub fn fraction_times_power_2(val: i128, exp: i128) -> Fraction {
        if exp >= 0 {
            let pow = 1 << exp;
            Fraction::new_from_int(
                if val != 1 { val * pow } else { pow }
            )
        } else {
            let pow = 1 << (-1 * exp);
            Fraction::new(
                val,
                pow
            )
        }
    }

    // Returns an aproximation of euler number, with 
    // 15 numbers decimal aproximation.
    pub fn euler_aprox() -> Fraction {
        Fraction::new(
            7437374403113, 
            2736057139200
        )
    }
}

