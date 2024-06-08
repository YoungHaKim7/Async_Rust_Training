#![feature(impl_trait_in_assoc_type)]
#[warn(deprecated_where_clause_location)]

use std::future::Future;

pub trait KvIterator {
    type NextFuture<'b>: Future<Output = Option<&'b [u8]>>
    where
        Self: 'b;
    fn next<'s>(&'s mut self) -> Self::NextFuture<'s>;
    //fn get_name<'s>(&'s self) -> &'s [u8];
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a [u8],
}

impl<'a> KvIterator for Person<'a> {
    type NextFuture<'b>
    where
        Self: 'b
    = impl Future<Output = Option<&'b [u8]>> where Self: 'b;
    fn next<'s>(&'s mut self) -> Self::NextFuture<'s> {
        async move { Some(self.name) }
    }
    // fn get_name<'s>(&'s self) -> &'s [u8] {
    //     self.name
    // }
}

fn main() {
    // Person(Self) outlive 'a
    {
        let mut p = Person { name: b"" };
        {
            let name = vec![97, 98, 99];
            p.name = name.as_slice();

            dbg!(p);
            dbg!(name);
            // let name1 = p.next().await;

            // let name1 = p.get_name();
            // dbg!(name1);
        }
    }
}
