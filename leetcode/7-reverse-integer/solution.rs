pub trait Reverse {
    fn rev(self) -> Self;
    fn checked_rev(self) -> Option<Self> where Self: Sized;
}

impl Reverse for i32 {
    fn rev(self) -> Self {
        let mut input = self;
        let mut rev = 0;
        while input != 0 {
            rev = rev * 10 + input % 10;
            input /= 10;
        }
        rev
    }
    
    fn checked_rev(self) -> Option<Self> {
        let mut input = self;
        let mut rev: Self = 0;
        while input != 0 {
            rev = rev.checked_mul(10)?.checked_add(input % 10)?;
            input /= 10;
        }
        Some(rev)
    }
}

impl Solution {
    pub fn reverse(n: i32) -> i32 {
        n.checked_rev().unwrap_or(0)
    }
}
