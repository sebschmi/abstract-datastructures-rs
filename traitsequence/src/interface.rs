use std::fmt::{Debug, Write};
use std::iter::FromIterator;
use std::ops::{Index, IndexMut, Range};

/// A type behaving like a sequence over the type `Item`.
pub trait Sequence<Item, Subsequence: Sequence<Item, Subsequence> + ?Sized>:
    Index<usize, Output = Item> + Index<Range<usize>, Output = Subsequence>
{
    /// The iterator type of the sequence.
    type Iterator<'a>: DoubleEndedIterator<Item = &'a Item>
    where
        Self: 'a,
        Item: 'a;

    /// Returns a prefix with length `len` of this sequence.
    /// Panics if `len >= self.len()`.
    fn prefix(&self, len: usize) -> &Subsequence {
        debug_assert!(len < self.len());
        &self[0..len]
    }

    /// Returns a suffix with length `len` of this sequence.
    /// Panics if `len >= self.len()`.
    fn suffix(&self, len: usize) -> &Subsequence {
        debug_assert!(len < self.len());
        &self[self.len() - len..self.len()]
    }

    /// Returns an iterator over the sequence.
    fn iter(&self) -> Self::Iterator<'_>;

    /// Returns the length of the sequence.
    fn len(&self) -> usize;

    /// Returns true if the sequence is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the first item of the sequence.
    fn first(&self) -> Option<&Item> {
        self.iter().next()
    }

    /// Returns the last item of the sequence.
    fn last(&self) -> Option<&Item> {
        self.iter().last()
    }

    /// Returns true if this is a proper subsequence of the given sequence.
    /// Proper means that the sequences are not equal.
    fn is_proper_subsequence_of(&self, other: &Self) -> bool
    where
        Item: Eq,
    {
        if self.len() >= other.len() {
            return false;
        }

        for start_index in 0..=other.len() - self.len() {
            let mut found_subsequence = true;
            for index in 0..self.len() {
                if self[index] != other[start_index + index] {
                    found_subsequence = false;
                    break;
                }
            }
            if found_subsequence {
                return true;
            }
        }

        false
    }

    /// Returns true if this sequence contains the given item.
    fn contains(&self, item: &Item) -> bool
    where
        Item: Eq,
    {
        self.iter().any(|i| item == i)
    }

    /// Returns an iterator over this sequence merged before the given other sequence under the assumption that the sequences can be merged this way.
    /// A merge is possible if a non-empty suffix of this sequence equals a non-empty prefix of the other sequence.
    ///
    /// The method panics if this sequence does not contain the first item of the other sequence or the other sequence is empty.
    /// The method does not fail if the sequences are not mergeable for other reasons.
    fn forward_merge_iter_assume_mergeable<'a>(
        &'a self,
        suffix: &'a Self,
    ) -> std::iter::Chain<Self::Iterator<'a>, std::iter::Skip<Self::Iterator<'a>>>
    where
        Item: Eq,
    {
        let first_item_index = self
            .iter()
            .enumerate()
            .filter(|(_, i)| *i == suffix.first().expect("The given sequence is empty."))
            .map(|(i, _)| i)
            .next()
            .expect("This sequence does not contain the first item of the given sequence.");
        self.iter()
            .chain(suffix.iter().skip(self.len() - first_item_index))
    }

    /// Returns an iterator over this sequence merged after the given other sequence under the assumption that the sequences can be merged this way.
    /// A merge is possible if a non-empty prefix of this sequence equals a non-empty suffix of the other sequence.
    ///
    /// The method panics if the other sequence does not contain the first item of this sequence or this sequence is empty.
    /// The method does not fail if the sequences are not mergeable for other reasons.
    fn backward_merge_iter_assume_mergeable<'a>(
        &'a self,
        suffix: &'a Self,
    ) -> std::iter::Chain<Self::Iterator<'a>, std::iter::Skip<Self::Iterator<'a>>>
    where
        Item: Eq,
    {
        suffix.forward_merge_iter_assume_mergeable(self)
    }

    /// Converts the sequence to a string using the debug formatting of the items.
    ///
    /// ```rust
    /// use traitsequence::interface::Sequence;
    ///
    /// let sequence = [0, 2, 1];
    /// debug_assert_eq!(sequence.to_debug_string(), "[0, 2, 1]".to_string());
    ///
    /// let sequence = ["a", "c", "b"];
    /// debug_assert_eq!(sequence.to_debug_string(), "[\"a\", \"c\", \"b\"]".to_string());
    /// ```
    fn to_debug_string(&self) -> String
    where
        Item: Debug,
    {
        let mut result = String::new();
        write!(result, "[").unwrap();
        let mut once = true;
        for item in self.iter() {
            if once {
                once = false;
            } else {
                write!(result, ", ").unwrap();
            }
            write!(result, "{:?}", item).unwrap();
        }
        write!(result, "]").unwrap();
        result
    }
}

/// A type behaving like a mutable sequence over the type `Item`.
///
/// That is, its items can be mutated, but the sequence it self can not.
/// For a sequence where items can be appended, rearranged etc. see [EditableSequence].
pub trait SequenceMut<Item, Subsequence: SequenceMut<Item, Subsequence> + ?Sized>:
    Sequence<Item, Subsequence>
    + IndexMut<usize, Output = Item>
    + IndexMut<Range<usize>, Output = Subsequence>
{
    /// The mutable iterator type of the sequence.
    type IteratorMut<'a>: Iterator<Item = &'a mut Item>
    where
        Self: 'a,
        Item: 'a;

    /// Returns a mutable iterator over the sequence.
    fn iter_mut(&mut self) -> Self::IteratorMut<'_>;
}

/// A type behaving like an owned sequence over the type `Item`.
/// Currently this only means the sequence is `Sized`.
pub trait OwnedSequence<Item, Subsequence: Sequence<Item, Subsequence> + ?Sized>:
    Sequence<Item, Subsequence> + Sized
{
}

/// A type behaving like an cloneable sequence over the type `Item`.
/// Currently this only means the sequence is `ToOwned`.
pub trait CloneableSequence<Item: Clone, Subsequence: CloneableSequence<Item, Subsequence> + ?Sized>:
    ToOwned
{
}

/// A type behaving like a sequence over the type `Item` that can be edited.
///
/// This sequences items can not necessarily be mutated themselves, but they can be rearranged or new items can be appended etc.
/// For a sequence where the items themselves can be mutated, see [SequenceMut].
pub trait EditableSequence<Item, Subsequence: Sequence<Item, Subsequence> + ?Sized>:
    Sequence<Item, Subsequence> + Extend<Item> + IntoIterator<Item = Item> + FromIterator<Item>
{
    /// Replace the item at the given index with the given item.
    fn set(&mut self, index: usize, item: Item);

    /// See [Vec::split_off].
    fn split_off(&mut self, at: usize) -> Self;

    /// Extend this sequence from a sequence of compatible items.
    fn extend_into<
        ExtensionItem: Into<Item>,
        ExtensionSource: IntoIterator<Item = ExtensionItem>,
    >(
        &mut self,
        extension: ExtensionSource,
    ) {
        self.extend(extension.into_iter().map(Into::into));
    }

    /// Reserve memory for at least `additional` items.
    fn reserve(&mut self, additional: usize);

    /// Resize to contain the given number of items.
    /// Empty spaces are filled with the given item.
    fn resize(&mut self, new_len: usize, value: Item)
    where
        Item: Clone;

    /// Resize to contain the given number of items.
    /// Empty spaces are filled with the given items generated by `generator`.
    fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> Item);

    /// Insert the given item at the end of the sequence.
    fn push(&mut self, item: Item);

    /// Delete the items in the specified range.
    fn delete(&mut self, range: Range<usize>)
    where
        Item: Clone,
    {
        assert!(range.end <= self.len());
        if range.start >= range.end {
            assert_eq!(range.start, range.end);
        } else {
            for index in range.start..self.len() - range.len() {
                self.set(index, self[index + range.len()].clone());
            }
            self.resize_with(self.len() - range.len(), || unreachable!());
        }
    }

    /// Insert a repeat at `target` that consists of the cloned items in `source_range`.
    fn insert_repeat(&mut self, source_range: Range<usize>, target: usize)
    where
        Item: Clone,
    {
        assert!(source_range.end <= self.len());
        if source_range.start >= source_range.end {
            assert_eq!(source_range.start, source_range.end);
        } else {
            self.resize(self.len() + source_range.len(), self[0].clone());
            for index in (target + source_range.len()..self.len() + source_range.len()).rev() {
                self.set(index, self[index - source_range.len()].clone());
            }
            for index in 0..source_range.len() {
                if index + source_range.start < target {
                    self.set(index + target, self[index + source_range.start].clone());
                } else {
                    self.set(index + target, self[index + source_range.end].clone());
                }
            }
        }
    }

    /// See [`Vec::splice`].
    ///
    /// This definition does not return an iterator of the removed characters.
    fn splice(&mut self, range: Range<usize>, replace_with: impl IntoIterator<Item = Item>);
}

#[cfg(test)]
mod tests {
    use crate::interface::Sequence;

    #[test]
    fn test_merge_sequences_simple() {
        let s1 = vec![0, 1, 2, 3, 4, 5];
        let s2 = vec![3, 4, 5, 6, 7, 8];
        let merged: Vec<_> = s1
            .forward_merge_iter_assume_mergeable(&s2)
            .copied()
            .collect();
        debug_assert_eq!(merged, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
