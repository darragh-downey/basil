use std::fs::File;
use std::io::BufReader;

pub fn open_input(file_path: &str) -> std::io::BufReader<std::fs::File> {
    let file = match File::open(file_path) {
        Err(why) => panic!("Error opening input file {}", why),
        Ok(file) => file,
    };
    BufReader::new(file)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}