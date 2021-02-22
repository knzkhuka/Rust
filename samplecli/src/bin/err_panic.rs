fn get_from_file() -> i32 {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).expect("failed to open the file.");
    let ret = num_str
        .trim()
        .parse::<i32>()
        .expect("failed to prase to a number.");
    ret * 2
}
fn main() {
    println!("{}", get_from_file());
}
