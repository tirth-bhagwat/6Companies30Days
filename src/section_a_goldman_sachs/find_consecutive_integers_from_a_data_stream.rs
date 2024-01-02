// https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/

struct Solution {}

struct DataStream {
    pub count: i32,
    pub k: i32,
    pub value: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        DataStream { count: 0, k, value }
    }

    fn consec(&mut self, num: i32) -> bool {
        if num == self.value {
            self.count += 1;
        } else {
            self.count = 0
        }

        if self.count >= self.k {
            return true;
        }

        false
    }
}
