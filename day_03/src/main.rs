use std::fs;
use crate::CellType::*;
use crate::Validity::*;


#[derive(Clone, PartialEq, Eq, Hash)]
enum Validity {
    VALID,
    INVALID,
}


#[derive(Clone, PartialEq, Eq, Hash)]
enum CellType {
    IGNORE,
    NUMBER,
    SYMBOL,
}

impl CellType {
    pub fn fmt(&self) -> String {
        match self {
            IGNORE => { String::from("Ignore") }
            NUMBER => { String::from("Number") }
            SYMBOL => { String::from("Symbol") }
        }
    }
}


#[derive(Clone, PartialEq, Eq, Hash)]
struct Cell {
    m: usize,
    n: usize,
    kind: CellType,
    number: u32,
    symbol: String,
    valid: Validity,
}

impl Cell {
    pub fn new(m: usize, n: usize, reading: String) -> Self {
        if reading == String::from(".") {
            return Self {
                m,
                n,
                kind: IGNORE,
                number: 0,
                symbol: String::from(""),
                valid: INVALID,
            };
        }

        match reading.parse() {
            Ok(value) => {
                return Self {
                    m,
                    n,
                    kind: NUMBER,
                    number: value,
                    symbol: String::from(""),
                    valid: INVALID,
                };
            }
            _ => {}
        };

        return Self {
            m,
            n,
            kind: SYMBOL,
            number: 0,
            symbol: reading,
            valid: INVALID,
        };
    }

    pub fn new_valid_index_cell(m: usize, n: usize, number: u32) -> Self {
        Self {
            m,
            n,
            kind: NUMBER,
            number,
            symbol: String::from(""),
            valid: INVALID,
        }
    }
}

#[derive(Clone)]
struct Engine {
    raw_matrix: Vec<Vec<Cell>>,
    valid_numbers: Vec<Cell>,
    m: usize,
    n: usize,
}

impl Engine {
    pub fn new(matrix: Vec<Vec<Cell>>) -> Self {
        let raw_matrix: Vec<Vec<Cell>> = matrix.clone();
        let m: usize = raw_matrix.iter().count() - 1;
        let n: usize = raw_matrix[0].iter().count() - 1;
        Self {
            raw_matrix,
            valid_numbers: vec![],
            m,
            n,
        }
    }

    /// A(m-1, n-1)
    fn a0(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == 0 {
            return None;
        }
        if *n == 0 {
            return None;
        }

        Some(self.raw_matrix[*m - 1][*n - 1].clone())
    }

    /// A(m-1, n)
    fn a1(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == 0 {
            return None;
        }

        Some(self.raw_matrix[*m - 1][*n].clone())
    }

    /// A(m-1, n+1)
    fn a2(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == 0 {
            return None;
        }
        if *n == self.n {
            return None;
        }
        Some(self.raw_matrix[*m - 1][*n + 1].clone())
    }

    /// A(m, n-1)
    fn a3(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *n == 0 {
            return None;
        }
        Some(self.raw_matrix[*m][*n - 1].clone())
    }

    /// A(m, n)
    fn a4(&self, m: &usize, n: &usize) -> Option<Cell> {
        Some(self.raw_matrix[*m][*n].clone())
    }

    /// A(m, n+1)
    fn a5(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *n == self.n {
            return None;
        }
        Some(self.raw_matrix[*m][*n + 1].clone())
    }

    /// A(m+1, n-1)
    fn a6(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == self.m {
            return None;
        }
        if *n == 0 {
            return None;
        }
        Some(self.raw_matrix[*m + 1][*n - 1].clone())
    }

    /// A(m+1, n)
    fn a7(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == self.m {
            return None;
        }
        Some(self.raw_matrix[*m + 1][*n].clone())
    }

    /// A(m+1, n+1)
    fn a8(&self, m: &usize, n: &usize) -> Option<Cell> {
        if *m == self.m {
            return None;
        }
        if *n == self.n {
            return None;
        }
        Some(self.raw_matrix[*m + 1][*n + 1].clone())
    }

    fn get_neighbours(&self, m: &usize, n: &usize) -> Vec<Option<Cell>> {
        vec![
            self.a0(&m, &n),
            self.a1(&m, &n),
            self.a2(&m, &n),
            self.a3(&m, &n),
            self.a5(&m, &n),
            self.a6(&m, &n),
            self.a7(&m, &n),
            self.a8(&m, &n),
        ]
    }

    fn evaluate(&self, cell: &Cell) -> Validity {
        if cell.kind != NUMBER {
            return INVALID;
        }

        let neighbours: Vec<Option<Cell>> = self.get_neighbours(&cell.m, &cell.n);

        for neighbour in neighbours {
            match neighbour {
                None => {}
                Some(cell) => {
                    if cell.kind == SYMBOL {
                        return VALID;
                    }
                }
            }
        }
        INVALID
    }

    pub fn process_raw(&mut self) {
        let _matrix = self.raw_matrix.clone();
        for (_, raw) in _matrix.into_iter().enumerate() {
            for (_, cell) in raw.into_iter().enumerate() {
                if self.evaluate(&cell) == VALID {
                    self.push_valid_number(
                        Cell::new_valid_index_cell(
                            cell.m,
                            self.parse_first_number_col(&cell),
                            cell.number,
                        )
                    );
                }
            }
        }
    }


    pub fn valid_count(&self) -> usize {
        self.valid_numbers.iter().count()
    }
    pub fn parse_first_number_col(&self, cell: &Cell) -> usize {
        let mut valid_number_index = cell.n;

        for col in (0..cell.n + 1).rev() {
            let temp: Cell = self.raw_matrix[cell.m][col].clone();
            if temp.kind == NUMBER {
                valid_number_index = col;
            } else {
                break;
            }
        }
        valid_number_index
    }

    pub fn push_valid_number(&mut self, cell: Cell) {
        let filtered_set = self.valid_numbers.iter()
            .filter(|val| val.m == cell.m)
            .filter(|val| val.n == cell.n);

        let filtered_set_count = filtered_set.count();
        if filtered_set_count > 0 {
            return;
        }
        self.valid_numbers.push(cell.clone());
    }

    pub fn parse_full_number(&self, cell: Cell) -> u32 {
        let mut num: Vec<u32> = vec![];

        for col in cell.n..self.n + 1 {
            let temp: Cell = self.raw_matrix[cell.m][col].clone();
            if temp.kind == NUMBER {
                num.push(temp.number);
            } else {
                break;
            }
        }

        let number: String = num.iter().map(|&x| { x.to_string() }).collect();

        match number.parse() {
            Ok(num) => {
                return num;
            }
            _ => {}
        }
        0
    }

    pub fn calculate_sum(&self) -> u32 {
        println!("Total number of valid numbers : {}", self.valid_count());
        let mut array_of_numbers = vec![];
        for index_cell in &self.valid_numbers {
            let number = self.parse_full_number(index_cell.clone());
            println!("Valid number: {}", number);
            array_of_numbers.push(number);
        }

        array_of_numbers.iter().sum()
    }
}

fn main() {
    println!("<Day 02>");
    let file_contents = fs::read_to_string("data/003_data.txt").expect("Failed to read data");

    let mut matrix: Vec<Vec<Cell>> = vec![];

    for (x, line) in file_contents.split("\n").enumerate()
    {
        let mut raw: Vec<Cell> = vec![];
        for (y, cell) in line.split("").filter(|val| val != &"").enumerate()
        {
            raw.push(Cell::new(x, y, cell.to_string()))
        }
        matrix.push(raw);
    }

    let mut engine: Engine = Engine::new(matrix);

    engine.process_raw();

    let grand_sum = engine.calculate_sum();

    println!("Grand sum (method 2) : {}", grand_sum);
}