/*
https://leetcode.com/problems/two-sum/description/

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:
Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:
Input: nums = [3,3], target = 6
Output: [0,1]
*/
pub fn calculate(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut vec = Vec::new();

  for (position_x, number_x) in nums.iter().enumerate() {
    for position_y in position_x + 1..=nums.len() - 1 {
      if number_x + nums[position_y] == target {
        vec.push(position_x as i32);
        vec.push(position_y as i32);
        return vec;
      }
    }
  }

  Vec::new()
}