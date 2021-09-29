
pub struct Rnd { seed: u64 }
impl Rnd {
    pub fn new(seed: u64) -> Rnd {
        Rnd { seed }
    }
    pub fn rnd(&mut self) -> u64 {
        let r = self.seed;
        self.seed = (1103515245 * r + 12345) % (1 << 31);
        r
    }
    pub fn rnd_num(&mut self, max: u64) -> u64 {
        self.rnd() % max
    }

    #[allow(dead_code)]
    pub fn rnd_num_range(&mut self, min: u64, max: u64) -> u64 {
        let r = max - min;
        (self.rnd() % r) + min
    }
}
impl Iterator for Rnd {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.rnd())
    }
}



#[test]
fn test_rnd() {
    let rnd = Rnd::new(1);
    let numbers: Vec<_> = rnd.take(1000).collect();
    assert_eq!([1, 1103527590, 377401575], &numbers[0..3]);
}
