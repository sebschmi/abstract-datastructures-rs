use crate::interface::{CloneableSequence, EditableSequence, OwnedSequence, Sequence, SequenceMut};

impl<Item> Sequence<Item, [Item]> for Vec<Item> {
    type Iterator<'a> = std::slice::Iter<'a, Item> where Item: 'a;
    fn iter(&self) -> Self::Iterator<'_> {
        self[..].iter()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<Item> SequenceMut<Item, [Item]> for Vec<Item> {
    type IteratorMut<'a> = std::slice::IterMut<'a, Item> where Item: 'a;

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self[..].iter_mut()
    }
}

impl<Item> OwnedSequence<Item, [Item]> for Vec<Item> {}

impl<Item: Clone> CloneableSequence<Item, [Item]> for Vec<Item> {}

impl<Item> EditableSequence<Item, [Item]> for Vec<Item> {
    fn split_off(&mut self, at: usize) -> Self {
        Vec::split_off(self, at)
    }
}
