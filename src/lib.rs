// Define a point on the circle as a signle numbers representing its position on the segment
type Point = u128;

// Define an interval on the circle as a pair of points
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interval(Point, Point);

impl Interval {
    fn not_through_starting_point(&self) -> bool {
        self.0 < self.1
    }

    fn through_starting_point(&self) -> bool {
        self.0 > self.1
    }

    // Check if an interval is empty
    fn is_empty(&self) -> bool {
        self.0 == self.1
    }

    // Check if an interval is the entire circle
    fn is_full(&self) -> bool {
        self.1 == u128::MAX - self.0
    }

    // Check if an interval is a proper interval (i.e., not empty or the entire circle)
    fn is_interval(&self) -> bool {
        self.0 != self.1 && self.1 != u128::MAX - self.0
    }

    // Compute the union of two intervals
    pub fn union(&self, other: &Interval) -> Option<Interval> {
        if self.is_empty() {
            Some(other.clone())
        } else if other.is_empty() {
            Some(self.clone())
        } else if self.is_full() || other.is_full() {
            Some(Interval(0, u128::MAX))
        } else if self.is_interval() && other.is_interval() {
            if self.not_through_starting_point() && other.not_through_starting_point() {
                if self.0 <= other.0 {
                    if self.1 >= other.0 {
                        Some(Interval(self.0, self.1.max(other.1)))
                    } else {
                        None
                    }
                } else {
                    if other.1 >= self.0 {
                        Some(Interval(other.0, self.1.max(other.1)))
                    } else {
                        None
                    }
                }
            } else if self.through_starting_point() && other.not_through_starting_point() {
                Self::union_through_starting_point_not_through_starting_point(self, other)
            } else if self.not_through_starting_point() && other.through_starting_point() {
                Self::union_through_starting_point_not_through_starting_point(other, self)
            } else {
                Some(Interval(self.0.min(other.0), self.1.max(other.1)))
            }
        } else {
            None
        }
    }

    pub fn union_through_starting_point_not_through_starting_point(
        through_starting_point: &Interval,
        not_through_starting_point: &Interval,
    ) -> Option<Interval> {
        if not_through_starting_point.0 <= through_starting_point.1 {
            if not_through_starting_point.1 >= through_starting_point.0 {
                Some(Interval(0, u128::MAX))
            } else {
                Some(Interval(
                    through_starting_point.0,
                    through_starting_point.1.max(not_through_starting_point.1),
                ))
            }
        } else {
            None
        }
    }
}

// Compute the union of a list of intervals
pub fn union_list(intervals: &[(Interval, Interval)]) -> Vec<Option<Interval>> {
    intervals
        .iter()
        .map(|(interval_a, interval_b)| interval_a.union(interval_b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_list_empty() {
        let intervals: &[(Interval, Interval)] = &[];
        let expected: Vec<Option<Interval>> = vec![];
        assert_eq!(union_list(intervals), expected);
    }

    #[test]
    fn test_union_list_single() {
        let intervals = &[(Interval(0, 20), Interval(20, 30))];
        let expected = vec![Some(Interval(0, 30))];
        assert_eq!(union_list(intervals), expected);
    }

    #[test]
    fn test_union_list_multiple() {
        let intervals = &[
            (Interval(0, 20), Interval(20, 30)),
            (Interval(5, 30), Interval(25, 35)),
            (Interval(40, 50), Interval(60, 70)),
        ];
        let expected = vec![Some(Interval(0, 30)), Some(Interval(5, 35)), None];
        assert_eq!(union_list(intervals), expected);
    }

    #[test]
    fn test_union_list_empty_interval() {
        let intervals = &[
            (Interval(0, 10), Interval(20, 30)),
            (Interval(5, 5), Interval(25, 35)),
            (Interval(40, 50), Interval(60, 70)),
        ];
        let expected = vec![Some(Interval(0, 30)), None, None];
        assert_eq!(union_list(intervals), expected);
    }

    #[test]
    fn test_union_list_full_interval() {
        let intervals = &[
            (Interval(0, 10), Interval(9, 8)),
            (Interval(30, 23), Interval(20, 30)),
            (Interval(40, 50), Interval(48, 42)),
        ];
        let expected = vec![
            Some(Interval(0, u128::MAX)),
            Some(Interval(0, u128::MAX)),
            Some(Interval(0, u128::MAX)),
        ];
        assert_eq!(union_list(intervals), expected);
    }

    #[test]
    fn test_union_list_improper_interval() {
        let intervals = &[
            (Interval(0, 7), Interval(10, 20)),
            (Interval(10, 15), Interval(20, 30)),
            (Interval(20, 25), Interval(40, 10)),
            (Interval(40, 50), Interval(60, 70)),
        ];
        let expected = vec![None, None, None, None];
        assert_eq!(union_list(intervals), expected);
    }
}
