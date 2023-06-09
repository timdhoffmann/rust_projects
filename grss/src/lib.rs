#![allow(unused_must_use)]

pub fn find_matches(reader: impl std::io::BufRead, pattern: &str, mut writer: impl std::io::Write) {
    for mut line in reader.lines() {
        if line.as_mut().unwrap().contains(pattern) {
            // TODO: add error handling.
            writeln!(writer, "{}", line.unwrap());
        }
    }
}