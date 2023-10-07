/*
https://leetcode.com/problems/fizz-buzz/description

Given an integer n, return a string array answer (1-indexed) where:
  answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
  answer[i] == "Fizz" if i is divisible by 3.
  answer[i] == "Buzz" if i is divisible by 5.
  answer[i] == i (as a string) if none of the above conditions are true.

Example 1:
  Input: n = 3
  Output: ["1","2","Fizz"]

Example 2:
  Input: n = 5
  Output: ["1","2","Fizz","4","Buzz"]

Example 3:
  Input: n = 15
  Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
*/

pub fn format(n: i32) -> Vec<String> {
  let mut vec: Vec<String> = Vec::new();

  for number in 1..=n {

    match (number % 5, number % 3) {
      (0, 0) => vec.push("FizzBuzz".to_string()),
      (_, 0) => vec.push("Fizz".to_string()),
      (0, _) => vec.push("Buzz".to_string()),
      _ => vec.push(number.to_string())
    }
  }

  return vec;
}