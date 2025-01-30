use crate::interface::{CloneableSequence, EditableSequence, OwnedSequence, Sequence, SequenceMut};

impl<Item> Sequence<Item, [Item]> for Vec<Item> {
    type Iterator<'a>
        = std::slice::Iter<'a, Item>
    where
        Item: 'a;
    fn iter(&self) -> Self::Iterator<'_> {
        self[..].iter()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<Item> SequenceMut<Item, [Item]> for Vec<Item> {
    type IteratorMut<'a>
        = std::slice::IterMut<'a, Item>
    where
        Item: 'a;

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self[..].iter_mut()
    }
}

impl<Item> OwnedSequence<Item, [Item]> for Vec<Item> {}

impl<Item: Clone> CloneableSequence<Item, [Item]> for Vec<Item> {}

impl<Item> EditableSequence<Item, [Item]> for Vec<Item> {
    fn set(&mut self, index: usize, item: Item) {
        self[index] = item;
    }

    fn split_off(&mut self, at: usize) -> Self {
        Vec::split_off(self, at)
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn resize(&mut self, new_len: usize, value: Item)
    where
        Item: Clone,
    {
        self.resize(new_len, value);
    }

    fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> Item) {
        self.resize_with(new_len, generator);
    }

    fn push(&mut self, item: Item) {
        self.push(item)
    }

    fn splice(
        &mut self,
        range: std::ops::Range<usize>,
        replace_with: impl IntoIterator<Item = Item>,
    ) {
        self.splice(range, replace_with);
    }
}
