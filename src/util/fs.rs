use std::fs;

pub fn read_rub_file(path: &str) -> String {
    let file_content = fs::read_to_string(path);
    match file_content {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    }
}

pub fn slice_line(raw_content: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = vec![];
    let raws: Vec<&str> = raw_content.split("\n").collect();
    for s in raws {
        // Remove empty string
        if s == "" {
            continue;
        };
        lines.push(s);
    }
    lines
}
