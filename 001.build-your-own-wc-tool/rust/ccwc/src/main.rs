fn main() {
    let option = std::env::args().nth(1).expect("Option (-c, -l or -w) not provided");
    let file_path = std::env::args().nth(2).expect("File path not provided");

    let content = std::fs::read_to_string(&file_path).expect("could not read file");

    if option == "-c" {
        println!("{} {}", byte_counts(content), file_path)
    } else if option == "-l" {
        println!("{} {}", line_counts(content), file_path)
    } else if option == "-w" {
        println!("{} {}", line_counts(content), file_path)
    }

}

fn byte_counts(input_text: String) -> usize {
    return input_text.as_bytes().len();
}

fn line_counts(input_text: String) -> usize {
    return input_text.lines().count();
}

// fn word_counts(input_text: String) -> usize {
// }
