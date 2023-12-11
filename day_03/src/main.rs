use std::fs;

enum Cell {
    IGNORE,
    NUMBER(u32),
    SYMBOL(String),
}

impl Cell {
    pub fn new(reading: String) -> Self {
        if reading == String::from(".") {
            return Cell::IGNORE;
        }

        match reading.parse() {
            Ok(value) => {
                return Cell::NUMBER(value);
            }
            _ => {}
        };

        Cell::SYMBOL(reading)
    }
}

fn main() {
    println!("<Day 02>");
    let file_contents = fs::read_to_string("data/003_data.txt").expect("ok");

    let mut matrix: Vec<Vec<Cell>> = vec![];

    for (raw, line) in file_contents.split("\n").enumerate() {
        let mut raw: Vec<Cell> = vec![];
        for (col, cell) in line.
            split("").
            filter(|val| val != &"").
            enumerate()
        {
            let _cell = Cell::new(cell.to_string());
            raw.push(_cell)
        }
        matrix.push(raw);
    }

    println!("completed matrix formation!");
}
