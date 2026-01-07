use core::num;


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            nums.iter().enumerate().map(move |(j, y)| (i, j, x, y))
        })
        .find_map(|(i, j, x, y)| {
            if (x + y == target) {
                Some(vec![i as i32, j as i32])
            } else {
                None
            }
        }).unwrap()
}

fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut found: Option<Vec<i32>> = None;

    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if x + y == target {
                found = Some(vec![i as i32, j as i32]);
            }
        }
    };

    found.unwrap()
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum_v2(nums, target);
    println!("{:?}", result); // Output should be [0, 1]
}
