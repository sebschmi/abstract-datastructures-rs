use crate::interface::{CloneableSequence, Sequence, SequenceMut};

impl<Item> Sequence<Item, [Item]> for [Item] {
    type Iterator<'a> = std::slice::Iter<'a, Item> where Item: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        <[Item]>::iter(self)
    }

    fn len(&self) -> usize {
        <[Item]>::len(self)
    }
}

impl<Item> SequenceMut<Item, [Item]> for [Item] {
    type IteratorMut<'a> = std::slice::IterMut<'a, Item> where Item: 'a;
    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        <[Item]>::iter_mut(self)
    }
}

impl<Item: Clone> CloneableSequence<Item, [Item]> for [Item] {}

#[cfg(test)]
mod tests {
    use crate::interface::Sequence;

    #[test]
    fn test_len() {
        let array = [0, 1, 2];
        let slice = &array[0..2];
        // Making sure that the fully qualified syntax in the trait implementation works as I think and does not create endless recursion.
        debug_assert_eq!(2, Sequence::len(slice));
    }
}
