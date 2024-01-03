// https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/

use std::collections::HashMap;

struct Solution {}
struct DP {
    pub dp: HashMap<(i16, i16), i32>,
}

impl DP {
    const MOD: i32 = 10_i32.pow(9) + 7;
    pub fn find_ways(&mut self, start: i16, end: i16, k: i16) -> i32 {
        let diff = start.abs_diff(end) as i16;

        if diff == k {
            return 1;
        }

        let mut tmp = 0;

        if let Some(val) = self.dp.get(&(k, diff)) {
            return *val as i32;
        } else {
            tmp = (self.find_ways(start - 1, end, k - 1) + self.find_ways(start + 1, end, k - 1))
                % DP::MOD;
            self.dp.insert((k, diff), tmp);
        }

        return tmp;
    }
}

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        let diff = start_pos.abs_diff(end_pos) as i32;
        match (diff % 2 == 0, k % 2 == 0) {
            (true, false) | (false, true) => {
                return 0;
            }
            (_, _) => {}
        }

        if diff > k {
            return 0;
        }

        let mut obj = DP { dp: HashMap::new() };

        let mut dp: HashMap<(i16, i16), u64> = HashMap::new();

        obj.dp.insert((1, 1), 1);
        obj.dp.insert((2, 0), 2);

        obj.find_ways(start_pos as i16, end_pos as i16, k as i16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_ways(1, 2, 3), 3, "Test1")
    }

    // #[test]
    // fn trial_func() {
    //     for i in 0..=10 {
    //         println!("{}", Solution::number_of_ways(1, 2, i));
    //     }
    //     assert!(false);
    // }
}
