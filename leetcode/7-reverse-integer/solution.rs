pub fn reverse_string(string: String) -> String {
    return string.chars().rev().collect();
}

impl Solution {
    pub fn reverse(n: i32) -> i32 {
        let reversed = reverse_string(n.abs().to_string()).parse::<i32>().unwrap_or(0);
        if n > 0 { reversed } else { -reversed }
    }
}
