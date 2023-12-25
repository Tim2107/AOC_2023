pub fn subtract_tuples(a: (usize, usize), b: (usize, usize)) -> (isize, isize) {
    let dx = a.0 as isize - b.0 as isize;
    let dy = a.1 as isize - b.1 as isize;

    (dx, dy)
}