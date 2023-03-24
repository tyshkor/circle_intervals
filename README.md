# circle_intervals

## The model and explaniations

The circle is represented as a segments with `u128::MAX` + 1 points (for simplicity's sake) where point 0 and point `u128::MAX` are "glued" together, in other words they are the same point.

An interval therefore is a tuple, organised naturally clockwise, that is, first element of which is included, the second is not.
An empty interval therefore is an interval both of its elements being the same value.
A full interval therefore is an interval which has the sum of its points equal to `u128::MAX`.
An interval that has its starting point bigger than its ending point represents the fact that al the points from the starting point upto and including the `u128::MAX` point are part of the interval as well as all points from `0` upto the ending point.
