use bs_num::{max, min, Numeric, Zero};
use std::fmt::Debug;

pub trait Coordinate: Copy + Clone + PartialEq + Debug {
    ///numeric type
    type Scalar: Numeric;

    ///dimension of coordinate
    const DIM: usize;

    /// creates coordinate with values from each dimension
    /// dim_val(i) -> returns coordinate value in ith dimension
    fn gen(dim_val: impl Fn(usize) -> Self::Scalar) -> Self;

    ///value in ith dim
    fn nth(&self, i: usize) -> Self::Scalar;

    ///mutable value in ith dim
    fn nth_mut(&mut self, i: usize) -> &mut Self::Scalar;

    fn new_origin() -> Self {
        Self::new_from_value(Zero::zero())
    }

    fn new_from_value(v: Self::Scalar) -> Self {
        Self::gen(|_| v)
    }

    fn component_wise(
        &self,
        other: &Self,
        func: impl Fn(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self {
        Self::gen(|i| func(self.nth(i), other.nth(i)))
    }

    fn all_comp(&self, other: &Self, func: impl Fn(Self::Scalar, Self::Scalar) -> bool) -> bool {
        let mut bln = true;
        let mut i: usize = 0;
        while bln && i < Self::DIM {
            bln = func(self.nth(i), other.nth(i));
            i += 1;
        }
        bln
    }

    fn min_of_bounds(&self, other: &Self) -> Self {
        self.component_wise(other, min)
    }

    fn max_of_bounds(&self, other: &Self) -> Self {
        self.component_wise(other, max)
    }

    fn add(&self, other: &Self) -> Self {
        self.component_wise(other, |l, r| l + r)
    }

    fn comp(&self, other: &Self) -> Self {
        self.sub(other)
    }

    fn sub(&self, other: &Self) -> Self {
        self.component_wise(other, |l, r| l - r)
    }

    fn mult(&self, k: Self::Scalar) -> Self {
        self.map(|v| k * v)
    }

    fn map(&self, transform: impl Fn(Self::Scalar) -> Self::Scalar) -> Self {
        Self::gen(|i| transform(self.nth(i)))
    }

    fn fold(
        &self,
        start_val: Self::Scalar,
        func: impl Fn(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self::Scalar {
        let mut total = start_val;
        for i in 0..Self::DIM {
            total = func(total, self.nth(i))
        }
        total
    }

    fn square_length(&self) -> Self::Scalar {
        self.fold(Zero::zero(), |acc, v| acc + (v * v))
    }

    fn square_distance(&self, other: &Self) -> Self::Scalar {
        self.comp(other).square_length()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    struct Pt<T>
        where
            T: Numeric,
    {
        x: T,
        y: T,
    }

    impl<T> Coordinate for Pt<T>
        where
            T: Numeric,
    {
        type Scalar = T;
        const DIM: usize = 2;

        fn gen(dim_val: impl Fn(usize) -> Self::Scalar) -> Self {
            Pt {
                x: dim_val(0),
                y: dim_val(1),
            }
        }

        fn nth(&self, i: usize) -> Self::Scalar {
            match i {
                0 => self.x,
                1 => self.y,
                _ => unreachable!(),
            }
        }

        fn nth_mut(&mut self, i: usize) -> &mut Self::Scalar {
            match i {
                0 => &mut self.x,
                1 => &mut self.y,
                _ => unreachable!(),
            }
        }
    }

    fn even(x: i32) -> bool {
        x % 2 == 0
    }

    fn both_even(x: i32, y: i32) -> bool {
        even(x) && even(y)
    }


    #[test]
    fn test_pt_using_coordinates() {
        let mut pt = Pt::new_origin();
        *pt.nth_mut(0) = 3.;
        *pt.nth_mut(1) = 4.;

        println!("{:?}", pt);
        let a = Pt { x: 1.0, y: 1.0 };
        let b = Pt { x: 4.0, y: 5.0 };
        assert_eq!(pt.square_length(), 25.);
        assert_eq!(pt.square_length(), a.square_distance(&b));

        let a = Pt { x: 2.0, y: 2.0 };
        let b = Pt { x: 1.0, y: 7.0 };
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("a.min_of_bounds(b) = {:?}", a.min_of_bounds(&b));
        println!("a.max_of_bounds(b) = {:?}", a.max_of_bounds(&b));

        let a = Pt { x: 2.0, y: 2.0 };
        assert_eq!(a.mult(3.0), Pt { x: 6.0, y: 6.0 });
        assert_eq!(a.map(|x| x * 3.0), Pt { x: 6.0, y: 6.0 });

        let a = Pt { x: 2, y: 2 };
        let b = Pt { x: 8, y: 10 };
        assert!(a.all_comp( &b, both_even));
        let c = a.add(&b);
        assert_eq!(c, Pt { x: 10, y: 12 });
    }
}
