#![feature(generic_associated_types)]

trait Iterable {
    // Type of item yielded up; will be a reference into `Self`.
    type Item<'collection>
    where
        Self: 'collection;

    // Type of iterator we return. Will return `Self::Item` elements.
    type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
    where
        Self: 'collection;

    fn iter<'c>(&'c self) -> Self::Iterator<'c>;
    //           ^^                         ^^
    //
    // Returns a `Self::Iter` derived from `self`.
}

// from before
struct Iter<'c, T> {
    data: &'c [T],
}

impl<'c, T> Iterator for Iter<'c, T> {
    type Item = &'c T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((prefix_elem, suffix)) = self.data.split_first() {
            self.data = suffix;
            Some(prefix_elem)
        } else {
            None
        }
    }
}

impl<T> Iterable for [T] {
    type Item<'c> = &'c T
    where
        T: 'c;

    type Iterator<'c> = Iter<'c, T>
    where
        T: 'c;

    fn iter<'c>(&'c self) -> Self::Iterator<'c> {
        Iter { data: self }
    }
}

impl<T> Iterable for Vec<T> {
    type Item<'c> = &'c T
    where
        T: 'c;

    type Iterator<'c> = Iter<'c, T>
    where
        T: 'c;

    fn iter<'c>(&'c self) -> Self::Iterator<'c> {
        Iter { data: self }
    }
}

fn count_twice<I: Iterable + ?Sized>(iterator: &I) -> usize {
    let mut count = 0;

    for _ in iterator.iter() {
        count += 1;
    }

    for _ in iterator.iter() {
        count += 1;
    }

    count
}

fn main() {
    let x: &[usize] = &[1, 2, 3];
    let c = count_twice(x);
    assert_eq!(c, 6);
    println!("c = {c}");

    let c = count_twice(&vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(c, 12);
    println!("c = {c}");
}
