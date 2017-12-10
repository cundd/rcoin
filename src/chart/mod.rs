use std::collections::BTreeMap;

mod point;

pub use self::point::Point;

type PointRow<'a> = BTreeMap<usize, &'a Point>;

struct PointMatrix<'a> {
    rows: BTreeMap<usize, PointRow<'a>>,
}

pub struct Chart {
    width: usize,
    height: usize,
}

const BLOCK_FULL: &'static str = "\u{2588}";
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";


fn build_rows<'a>(chart: &Chart, points: Vec<&'a Point>) -> PointRow<'a> {
    let mut row = PointRow::new();

    for point in points {}

    row
}

fn sort_points<'a>(chart: &Chart, points: Vec<&'a Point>) -> PointMatrix<'a> {
    let mut rows: BTreeMap<usize, PointRow> = BTreeMap::new();

    for point in points {
        let mut handled = false;

        {
            let row_option = rows.get_mut(&point.y);
            if let Some(row) = row_option {
                row.insert(point.x, point);
                handled = true;
            }
        }
        if !handled {
            let mut row = PointRow::new();
            row.insert(point.x, point);
            rows.insert(point.y, row);
        }
    }

    PointMatrix { rows }
}


impl Chart {
    pub fn new(width: usize, height: usize) -> Self {
        Chart { width, height }
    }

    pub fn draw_points(&self, points: Vec<&Point>) {
//        print!("{}", BLOCK_UPPER_HALF);
//        print!("{}", BLOCK_LOWER_HALF);
//        print!("{}", BLOCK_FULL);

        let matrix = sort_points(self, points);

        for n in 0..self.height {
            if let Some(row) = matrix.rows.get(&n) {
                self.draw_row(row);
                println!();
            } else {
                println!();
            }
        }

//        for point in points {
//            self.draw_point(point)
//        }
    }

    pub fn draw_point(&self, point: &Point) {
        self.draw_points(vec![point]);
    }

    fn draw_row(&self, row: &PointRow) {
        for n in 0..self.width {
            if let Some(p) = row.get(&n) {
                print!("{}", BLOCK_FULL);
            } else {
                print!(" ");
            }
        }
    }
}
