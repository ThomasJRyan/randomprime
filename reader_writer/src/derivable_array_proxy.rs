use reader::{Reader, Readable};
use writer::Writable;

use std::marker::PhantomData;
use std::io::Write;
use std::borrow::Borrow;

/// Derivable Array Proxy - wraps an iterator for derived array.
///
/// The Readable::size of the wrapper is equal to the sum of all the items in
/// the wrapped iterator. Similarly, when using Writable::write, it calls each
/// the write method of each item in the wrapped iterator.
#[derive(Clone)]
pub struct Dap<I, T>(I, PhantomData<*const T>)
    where I: Iterator + Clone,
          I::Item: Borrow<T>;

impl<I, T> Dap<I, T>
    where I: Iterator + Clone,
          I::Item: Borrow<T>,
{
    pub fn new(i: I) -> Dap<I, T>
    {
        Dap(i, PhantomData)
    }
}

impl<'a, I, T> Readable<'a> for Dap<I, T>
    where I: Iterator + Clone,
          I::Item: Borrow<T>,
          T: Readable<'a>,
{
    type Args = ();
    fn read(_: Reader<'a>, (): ()) -> (Self, Reader<'a>)
    {
        panic!("Dap should not ever be read.")
    }

    fn size(&self) -> usize
    {
        self.0.clone().map(|t| t.borrow().size()).sum()
    }
}

impl<I, T> Writable for Dap<I, T>
    where I: Iterator + Clone,
          I::Item: Borrow<T>,
          T: Writable
{
    fn write<W: Write>(&self, writer: &mut W)
    {
        for e in self.0.clone() {
            e.borrow().write(writer)
        }
    }
}

impl<I, T> From<I> for Dap<I, T>
    where I: Iterator + Clone,
          I::Item: Borrow<T>,
{
    fn from(iter: I) -> Self
    {
        Dap::new(iter)
    }
}
