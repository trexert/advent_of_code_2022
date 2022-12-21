pub mod common_startup;
pub mod dijkstra;
pub mod floydwarshall;
pub mod interval;
pub mod logger;
pub mod op_wrapper;

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

// When called on a slice which is guaranteed to have p(t) == false for all values below a point,
// and p(t) == true above a point, find the first element for which p(t) == true
pub trait PositionBinary<T> {
    fn position_binary<P>(&self, p: P) -> Option<usize>
    where
        P: Fn(&T) -> bool;
}

impl<T> PositionBinary<T> for &[T] {
    fn position_binary<P>(&self, p: P) -> Option<usize>
    where
        P: Fn(&T) -> bool,
    {
        let (mut min, mut max) = (0, self.len());
        while min + 1 < max {
            let check = (min + max) / 2;
            if p(&self[check]) {
                max = check;
            } else {
                min = check + 1;
            }
        }
        if p(&self[min]) {
            Some(min)
        } else if p(&self[min + 1]) {
            Some(min + 1)
        } else {
            None
        }
    }
}
