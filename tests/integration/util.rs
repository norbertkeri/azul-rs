use std::fmt::Debug;
pub fn eq_lists<T>(a: &[T], b: &[T])
where
    T: PartialEq + Ord + Debug,
{
    let mut a: Vec<_> = a.iter().collect();
    let mut b: Vec<_> = b.iter().collect();
    a.sort();
    b.sort();

    pretty_assertions::assert_eq!(a, b);
}
