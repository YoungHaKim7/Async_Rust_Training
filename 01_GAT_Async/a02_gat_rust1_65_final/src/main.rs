use std::fmt::Debug;
use std::iter::Filter;

trait LendingIterator {
    type Item<'a>: Debug
    where
        Self: 'a;

    fn for_each<'a, F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'a>),
        Self: 'a;

    fn filter<'a, P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item<'a>) -> bool;
}

pub struct WindowsMut<'x, T> {
    slice: &'x mut [T],
}

impl<'x, T: std::fmt::Debug> LendingIterator for WindowsMut<'x, T, T: std::fmt::Debug> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn for_each<'a, F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'a>),
    {
        f(self.slice);
    }

    fn filter<'a, P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item<'a>) -> bool,
    {
        unimplemented!()
    }
}

fn print_items<I>(iter: I)
where
    I: for<'a> LendingIterator<Item<'a> = &'a mut [usize]>,
    for<'a> I::Item<'a>: Debug,
{
    iter.for_each(|item| {
        println!("{:?}", item);
    });
}

fn main() {
    let mut array = [0; 16];
    let slice = &mut array.clone();
    let windows = WindowsMut { slice };
    print_items::<WindowsMut<usize>>(windows);
}
