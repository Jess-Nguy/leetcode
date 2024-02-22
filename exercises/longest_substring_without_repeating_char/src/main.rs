mod solutions;

use solutions::solution1::*;
fn main() {
    let solution = Solution1::length_of_longest_substring("abcabcbb".to_string());
    println!("Hello, world! {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_3_length_subsequence() {
        assert_eq!(
            Solution1::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test_solution_1_length() {
        assert_eq!(
            Solution1::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test_solution_3_length_substring() {
        // the answer is "wke", with the length of 3.
        // longest substring is near the end of the string
        assert_eq!(
            Solution1::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_solution_0_length() {
        assert_eq!(Solution1::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_solution_4_length_substring() {
        // longest substring is near the beginning of the string
        assert_eq!(
            Solution1::length_of_longest_substring("meattheent".to_string()),
            4
        );
    }

    #[test]
    fn test_solution_5_length_substring() {
        // longest substring is in the middle of the string
        assert_eq!(
            Solution1::length_of_longest_substring("fratttrackkkkmax".to_string()),
            5
        );
    }
}
