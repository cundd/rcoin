use std::collections::HashMap;

mod point;

pub use self::point::Point;

type PointRow<'a> = HashMap<usize, &'a Point>;

struct PointMatrix<'a> {
    rows: HashMap<usize, PointRow<'a>>,
}

pub struct Chart {
    width: usize,
    height: usize,
}

const BLOCK_FULL: &'static str = "\u{2588}";
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";


fn build_rows<'a>(chart: &Chart, points: Vec<&'a Point>) -> PointRow<'a> {
    let mut row = PointRow::with_capacity(chart.width);

    for point in points {}

    row
}

fn sort_points<'a>(chart: &Chart, points: Vec<&'a Point>) -> PointMatrix<'a> {
    let mut temp_rows: HashMap<usize, PointRow> = HashMap::with_capacity(chart.height);

    for point in points {
        let mut handled = false;

        {
            let row_option = temp_rows.get_mut(&point.y);
            if let Some(row) = row_option {
                row.insert(point.x, point);
                handled = true;
            }
        }
        if !handled {
            let mut row = PointRow::with_capacity(chart.width);
            row.insert(point.x, point);
            temp_rows.insert(point.y, row);
        }
    }

    let mut rows: HashMap<usize, PointRow> = HashMap::with_capacity(chart.height);
    for (y, mut row) in temp_rows {
        row.sort_by(|point_a, point_b| point_a.x.cmp(&point_b.x));
        rows.insert(y, row);
    }

    PointMatrix { rows }
}


impl Chart {
    pub fn new(width: usize, height: usize) -> Self {
        Chart { width, height }
    }

    pub fn draw_points(&self, points: Vec<&Point>) {
        print!("{}", BLOCK_UPPER_HALF);
        print!("{}", BLOCK_LOWER_HALF);
        print!("{}", BLOCK_FULL);

        let matrix = sort_points(self, points);

        for (_row_number, row) in matrix.rows {
//            println!("{:?}",row);

            self.draw_row(row);
        }

//        for point in points {
//            self.draw_point(point)
//        }
    }

    fn draw_row(&self, row: PointRow) {
        for n in 0..self.width {
            for point in &row {
                if point.x == n {
                    println!("{}", n);
                }
            }
        }
    }
    fn draw_pixel(&self, row: PointRow) {
        for n in 0..self.width {
            for point in &row {
                if point.x == n {
                    println!("{}", n);
                }
            }
        }
    }

    pub fn draw_point(&self, point: &Point) {
        self.draw_points(vec![point]);
    }
}


