fn main() {
    let args: String = std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ");

        println!("{args}")
}
