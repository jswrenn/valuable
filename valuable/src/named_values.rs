use core::iter::{self, FusedIterator};

use crate::field::*;
use crate::*;

/// Access values for a struct's static fields
pub struct NamedValues<'a> {
    fields: &'a [NamedField<'a>],
    values: &'a [Value<'a>],
}

impl<'a> NamedValues<'a> {
    pub fn new(fields: &'a [NamedField<'a>], values: &'a [Value<'a>]) -> NamedValues<'a> {
        NamedValues { fields, values }
    }

    pub fn get(&self, field: &NamedField<'_>) -> Option<&Value<'_>> {
        use core::mem;

        let idx = (field as *const _ as usize - &self.fields[0] as *const _ as usize)
            / mem::size_of::<NamedField>();
        self.values.get(idx)
    }

    pub fn iter<'b>(&'b self) -> Iter<'a, 'b> {
        Iter {
            iter: self.fields.iter().enumerate(),
            values: self.values,
        }
    }
}

impl<'a, 'b> IntoIterator for &'b NamedValues<'a> {
    type Item = (&'b NamedField<'a>, &'b Value<'a>);
    type IntoIter = Iter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, 'b> {
    iter: iter::Enumerate<core::slice::Iter<'b, NamedField<'a>>>,
    values: &'a [Value<'a>],
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = (&'b NamedField<'a>, &'b Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(move |(i, field)| (field, &self.values[i]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(move |(i, field)| (field, &self.values[i]))
    }
}

impl ExactSizeIterator for Iter<'_, '_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Iter<'_, '_> {}
