trait IntoIsizeTuple {
    fn into_isize_tuple(self) -> (isize, isize);
}

impl IntoIsizeTuple for (usize, usize) {
    fn into_isize_tuple(self) -> (isize, isize) {
        (self.0 as isize, self.1 as isize)
    }
}

impl IntoIsizeTuple for (isize, isize) {
    fn into_isize_tuple(self) -> (isize, isize) {
        self
    }
}

impl IntoIsizeTuple for (usize, isize) {
    fn into_isize_tuple(self) -> (isize, isize) {
        (self.0 as isize, self.1)
    }
}

impl IntoIsizeTuple for (isize, usize) {
    fn into_isize_tuple(self) -> (isize, isize) {
        (self.0, self.1 as isize)
    }
}


pub fn subtract_tuples<T: IntoIsizeTuple>(a: T, b: T) -> (isize, isize) {
    let a = a.into_isize_tuple();
    let b = b.into_isize_tuple();
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    (dx, dy)
}

pub fn add_tuples<T: IntoIsizeTuple, U: IntoIsizeTuple>(a: T, b: U) -> (isize, isize) {
    let a = a.into_isize_tuple();
    let b = b.into_isize_tuple();
    let dx = a.0 + b.0;
    let dy = a.1 + b.1;

    (dx, dy)
}

pub fn choose_tuple(v: &[(usize, usize)]) -> (usize, usize) {
    if v.len() != 2 {
        panic!("Expected a vector with exactly two elements");
    }

    if v[0] < v[1] {
        v[0]
    } else {
        v[1]
    }
}