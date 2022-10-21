#[derive(Debug)]
pub struct RingCurser {
    pub begin: usize,
    pub end: usize,
    total_length: usize,
}

impl RingCurser {
    pub fn new(total_length: usize) -> Self {
        Self {
            begin: 0,
            end: 0,
            total_length: total_length - 1,
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.begin == self.end {
            None
        } else if self.begin == self.total_length {
            let p = self.begin;

            self.begin = 0;

            Some(p)
        } else {
            let p = self.begin;

            self.begin += 1;

            Some(p)
        }
    }

    pub fn push(&mut self) -> usize {
        if self.end == self.total_length {
            self.end = 0;
        } else {
            self.end += 1;
        }

        if self.begin == self.end {
            if self.begin == self.total_length {
                self.begin = 0;
            } else {
                self.begin += 1;
            }
        }

        self.end
    }

    pub fn total_length(&self) -> usize {
        self.total_length
    }

    pub fn is_empty(&self) -> bool {
        self.begin == self.end
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use crate::storage::RingCurser;

    #[test]
    fn test_ring_curser4() {
        let mut rc = RingCurser::new(6);

        println!("b: {}, e: {}", rc.begin, rc.end);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);
        let r = rc.push();
        println!("b: {}, e: {}, r: {}", rc.begin, rc.end, r);

        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
        let r = rc.pop();
        println!("b: {}, e: {}, pos: {:?}", rc.begin, rc.end, r);
    }
}
