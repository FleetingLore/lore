use std::io;

fn br() {
    println!("{}", "-".repeat(40));
}

fn main() {
    println!("[{:?}]", std::env::current_dir().unwrap());

    loop {
        println!("_");

        // input which file to check.
        // for example, input "".
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.as_str().trim();

        let root = lore::read_and_parse_file(input);
        br();
        println!("{}", root);
        br();
    }
}