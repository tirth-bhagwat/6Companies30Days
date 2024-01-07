// https://leetcode.com/problems/image-smoother/

struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_max = img.len();
        let c_max = img[0].len();
        let mut res = vec![vec![0; c_max]; r_max];

        let diff = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for r in 0..r_max {
            for c in 0..c_max {
                let mut sum = img[r][c];
                let mut count = 1;
                for (dc, dr) in diff {
                    let new_c = c as i32 + dc;
                    let new_r = r as i32 + dr;

                    if (new_c >= 0 && new_c < c_max as i32) && (new_r >= 0 && new_r < r_max as i32)
                    {
                        sum += img[new_r as usize][new_c as usize];
                        count += 1;
                    }

                    res[r][c] = sum / count;
                }
            }
        }

        res
    }
}
