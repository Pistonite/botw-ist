use num_traits::Zero;
use paste::paste;

#[layered_crate::import]
use processor::Flags;

macro_rules! signed_add_with_carry {
    ($sz: literal) => {
        paste! {
            pub fn [<signed_add_with_carry $sz>](op1: [<i $sz>], op2: [<i $sz>], carry_in: bool) -> Flags {
                // Perform an addition, and set the proper flags
                let raw_sum = op1 + op2 + if carry_in { 1 } else { 0 };

                // Determine carry (when unsigned addition is wrong)
                let uop1 = op1 as [<u $sz>];
                let uop2 = op2 as [<u $sz>];
                let mut unsigned_sum = uop1.checked_add(uop2);
                let mut carry = false;
                if let Some(sum) = unsigned_sum {
                    if carry_in {
                        unsigned_sum = sum.checked_add(1);
                        if unsigned_sum.is_none() {
                            carry = true;
                        }
                    }
                } else {
                    carry = true;
                }

                // Determine overflow (when signed addition is wrong)
                let mut signed_sum = op1.checked_add(op2);
                let mut overflow = false;
                if let Some(sum) = signed_sum {
                    if carry_in {
                        signed_sum = sum.checked_add(1);
                        if signed_sum.is_none() {
                            overflow = true;
                        }
                    }
                } else {
                    overflow = true;
                }

                Flags {
                    z: raw_sum == 0,
                    n: raw_sum < 0,
                    c: carry,
                    v: overflow,
                }
            }
        }
    }
}
signed_add_with_carry!(32);
signed_add_with_carry!(64);

pub(crate) fn get_nzcv_flags<T: Zero + PartialEq + PartialOrd + Ord + Eq + Copy>(
    result: T,
    xn_val: T,
    xm_val: T,
    did_borrow: bool,
) -> Flags {
    Flags {
        n: result < T::zero(),
        z: result == T::zero(),
        c: !did_borrow,
        v: (xn_val < T::zero() && xm_val > T::zero() && result > T::zero())
        || (xn_val > T::zero() && xm_val < T::zero() && result < T::zero()),
    }
}
