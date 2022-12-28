pub fn reverse(input: &String) -> String {
    input.chars().rev().collect()
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    } else {
        return (inspect_numbers(input), String::from("digit"));
    }
}

pub fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }

    return count;
}

pub fn create_project(directory: &String) -> &String {
    return directory;
}
