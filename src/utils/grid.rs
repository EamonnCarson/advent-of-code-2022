
type Point = (usize, usize);

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new_from_rows(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data
            .get(0)
            .and_then(|x| Some(x.len()))
            .expect("data isn't empty");
        let data: Vec<T> = data
            .into_iter()
            .flat_map(|vec| vec.into_iter())
            .collect();
        Self {height, width, data}
    }

    fn check_row_col(&self, row: usize, col: usize) {
        match (row < self.height, col < self.width) {
            (true, true) => {},
            (true, false) => panic!("col {} out of bounds {}", col, self.width),
            (false, true) => panic!("row {} out of bounds {}", row, self.height),
            (false, false) => panic!("Both row {} and col {} out of bounds {} and {}", row, col, self.height, self.width),
        }
    }

    fn get_ix(&self, row: usize, col: usize) -> usize {
        self.check_row_col(row, col);
        col + row * self.width
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let ix = self.get_ix(row, col);
        self.data[ix] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let ix = self.get_ix(row, col);
        &self.data[ix]
    }

    pub fn get_xy_point(&self, point: Point) -> &T {
        self.get(point.1, point.0)
    }

    pub fn get_yx_point(&self, point: Point) -> &T {
        self.get(point.0, point.1)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let ix = self.get_ix(row, col);
        &mut self.data[ix]
    }

    pub fn get_xy_point_mut(&mut self, point: Point) -> &mut T {
        self.get_mut(point.1, point.0)
    }

    pub fn get_yx_point_mut(&mut self, point: Point) -> &mut T {
        self.get_mut(point.0, point.1)
    }

}