pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => {
                brackets.push(c);
            }
            '}' | ']' | ')' => {
                if let Some(&last_bracket) = brackets.last() {
                    match (last_bracket, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') => {
                            brackets.pop();
                        }
                        _ => {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    brackets.is_empty()
}
