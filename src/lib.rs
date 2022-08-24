#[macro_export]
macro_rules! hlist {
    () => { $crate::Empty };
    ($x:expr) => { $crate::params![$x,] };
    ($x:expr, $($tok:tt)*) => { $crate::Argument($x, $crate::params![$($tok)*]) };
}

pub trait Hlist: Sized {
    const NEST: usize = 0;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { Self::NEST == 0 }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Empty;

impl Hlist for Empty {
    const NEST: usize = 0;
    fn len(&self) -> usize { Self::NEST }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Element<H, T>(H, T);

impl<H, T> Hlist for Element<H, T> where T: Hlist {
    const NEST: usize = <T as Hlist>::NEST + 1;
    fn len(&self) -> usize { Self::NEST }
}

impl<H, T> Element<H, T> {
    pub fn pop(self) -> (H, T) { (self.0, self.1) }
}
