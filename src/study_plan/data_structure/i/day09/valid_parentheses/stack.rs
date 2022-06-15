pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == ')' || c == '}' || c == ']' {
            let popped = stack.pop();
            if popped.is_none() || c as u8 - popped.unwrap() as u8 > 2 {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(s: &str, res: bool) {
        assert_eq!(is_valid(s.to_string()), res);
    }

    #[test]
    fn case1() {
        test("()", true);
    }

    #[test]
    fn case2() {
        test("()[]{}", true);
    }

    #[test]
    fn case3() {
        test("(]", false);
    }
}
