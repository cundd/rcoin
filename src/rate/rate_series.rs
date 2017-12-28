use super::Rate;

#[derive(Debug)]
pub struct RateSeries {
    max_len: usize,
    data: Vec<Rate>,
}

impl RateSeries {
    pub fn new(max_len: usize) -> Self {
        RateSeries {
            max_len,
            data: Vec::with_capacity(max_len),
        }
    }
    pub fn push(&mut self, rate: Rate) {
        if self.data.len() == self.max_len {
            self.data.remove(0);
        }
        self.data.push(rate);
    }

    pub fn data(&self) -> &Vec<Rate> {
        &self.data
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn last(&self) -> Option<&Rate> {
        self.data.last()
    }

    #[allow(unused)]
    pub fn last_mut(&mut self) -> Option<&mut Rate> { self.data.last_mut() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Currency;

    #[test]
    fn push_test() {
        let mut rs = RateSeries::new(2);
        rs.push(Rate::new(Currency::Bitcoin, 0.01, 0.02));
        rs.push(Rate::new(Currency::Bitcoin, 0.11, 0.12));

        assert_eq!(2, rs.len());

        rs.push(Rate::new(Currency::Bitcoin, 0.21, 0.22));
        assert_eq!(2, rs.len());
    }
}
