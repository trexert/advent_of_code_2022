pub struct LetterSet {
    backing_store: u64,
}

impl LetterSet {
    pub fn len(&self) -> usize {
        self.backing_store.count_ones() as usize
    }
}

impl From<&[char]> for LetterSet {
    fn from(input: &[char]) -> Self {
        let backing_store = input
            .iter()
            .fold(0, |acc, c| acc | (1 << (*c as u8 - 'a' as u8)));
        LetterSet { backing_store }
    }
}

impl FromIterator<char> for LetterSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        LetterSet {
            backing_store: iter
                .into_iter()
                .fold(0, |acc, c| acc | (1 << (c as u8 - 'a' as u8))),
        }
    }
}
