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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_left_adjacent_to() {
        let a = 1..3;
        let b = 3..5;
        assert!(a.is_left_adjacent_to(&b));
        assert!(!b.is_left_adjacent_to(&a));
    }

    #[test]
    fn test_is_right_adjacent_to() {
        let a = 1..3;
        let b = 3..5;
        assert!(!a.is_right_adjacent_to(&b));
        assert!(b.is_right_adjacent_to(&a));
    }

    #[test]
    fn test_overlaps_negative() {
        let a = 1..3;
        let b = 3..5;
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps() {
        let a = 1..4;
        let b = 3..5;
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_is_adjacent_to() {
        let a = 1..3;
        let b = 3..5;
        assert!(a.is_adjacent_to(&b));
        assert!(b.is_adjacent_to(&a));
    }

    #[test]
    fn test_overlaps_or_is_adjacent_to() {
        let a = 1..3;
        let b = 3..5;
        assert!(a.overlaps_or_is_adjacent_to(&b));
        assert!(b.overlaps_or_is_adjacent_to(&a));
    }

    #[test]
    fn test_overlaps_or_is_adjacent_to_negative() {
        let a = 1..2;
        let b = 3..5;
        assert!(!a.overlaps_or_is_adjacent_to(&b));
        assert!(!b.overlaps_or_is_adjacent_to(&a));
    }
}
