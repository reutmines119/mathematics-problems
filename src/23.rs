fn main() {
    // Example problem: Given an array of integers, find the maximum sum of any subarray.
    let nums = [34, -50, 42, 14, -5, 86];
    let max_sum = find_max_subarray_sum(&nums);
    println!("The maximum sum of any subarray is: {}", max_sum);
}

fn find_max_subarray_sum(nums: &[i32]) -> i32 {
    let mut current_sum = nums[0];
    let mut max_sum = nums[0];

    for num in nums.iter() {
        if *num > current_sum + num {
            current_sum = *num;
        } else {
            current_sum += num;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}
