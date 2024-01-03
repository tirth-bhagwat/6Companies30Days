// https://leetcode.com/problems/number-of-people-aware-of-a-secret/

struct Solution {}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut people = vec![0; (forget) as usize - 1];
        people.push(1);
        println!("{:?} {}", people, people.len());
        for i in 0..n - 1 {
            people.remove(0);

            people.push(
                people
                    .get(..(forget -delay )as usize)
                    .unwrap()
                    .into_iter()
                    .fold(0, |agg, x| (agg + *x) % 1000000007),
            );
            println!("  {:?} {}", people, people.len());
        }

        people.into_iter().fold(0, |agg, x| (agg + x) % 1000000007) % 1000000007
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    }
}
