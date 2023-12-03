use std::ops::Range;

pub trait Span {
    fn is_left_adjacent_to(&self, other: &Self) -> bool;
    fn is_right_adjacent_to(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
    fn is_adjacent_to(&self, other: &Self) -> bool {
        self.is_left_adjacent_to(other) || self.is_right_adjacent_to(other)
    }
    fn overlaps_or_is_adjacent_to(&self, other: &Self) -> bool {
        self.overlaps(other) || self.is_adjacent_to(other)
    }
}

impl<T> Span for Range<T>
where
    T: PartialEq + PartialOrd,
{
    fn is_left_adjacent_to(&self, other: &Self) -> bool {
        self.end == other.start
    }

    fn is_right_adjacent_to(&self, other: &Self) -> bool {
        self.start == other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}
