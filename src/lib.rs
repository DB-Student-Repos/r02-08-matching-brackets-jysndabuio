pub fn brackets_are_balanced(string: &str) -> bool {
    let mut container = Vec::new();

    for c in string.chars() {
        if c == '(' || c == '[' || c == '{' {
            container.push(c);
        } else if c == ')' {
            if container.pop() != Some('(') {
                return false;
            }
        } else if c == ']' {
            if container.pop() != Some('[') {
                return false;
            }
        } else if c == '}' {
            if container.pop() != Some('{') {
                return false;
            }
        }
    }

    container.is_empty()

}
