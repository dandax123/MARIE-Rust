use std::fs;
pub fn extract_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;
    let mut file_contents = Vec::new();
    for line in contents.lines() {
        if !line.is_empty() {
            file_contents.push(line.trim().to_string())
        }
    }
    Ok(file_contents)
}

fn remove_comments(lines: Vec<String>) -> Vec<String> {
    return lines
        .into_iter()
        .map(|x| x.split("/").nth(0).expect("msg").trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
}

pub fn extract_token(x: Vec<String>) -> String {
    let removed_comments = remove_comments(x);
    // println!("{:?}", removed_comments);
    let y: Vec<String> = removed_comments
        .iter()
        .map(|x| x.replace("\t", " "))
        .map(|x| {
            // println!("{}", x);
            let y: String = x
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string() + " ")
                .collect();
            y
        })
        .collect();
    let j = y.join("");
    j
}
