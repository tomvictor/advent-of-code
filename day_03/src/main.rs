use std::collections::HashSet;
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

    pub fn log(&self) {
        println!("{} x {} ({})", self.m, self.n, self.kind.fmt())
    }
}

#[derive(Clone)]
struct Engine {
    raw_matrix: Vec<Vec<Cell>>,
    linear_matrix: Vec<Cell>,
    valid_numbers: Vec<Cell>,
    m: usize,
    n: usize,
}

impl Engine {
    pub fn new(matrix: Vec<Vec<Cell>>) -> Self {
        let raw_matrix: Vec<Vec<Cell>> = matrix.clone();
        let linear_matrix: Vec<Cell> = matrix.into_iter().flatten().collect();
        let m: usize = raw_matrix.iter().count() - 1;
        let n: usize = raw_matrix[0].iter().count() - 1;
        Self {
            raw_matrix,
            linear_matrix,
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

    fn scan(&self, x: &usize, y: &usize) -> Validity {
        let m: usize = x.clone();
        let n: usize = y.clone();

        let neighbours: Vec<Option<Cell>> = self.get_neighbours(&m, &n);

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

    fn evaluate(&self, cell: &Cell) -> Validity {
        if cell.kind == NUMBER {
            if self.scan(&cell.m, &cell.n) == VALID {
                return VALID;
            }
        }
        INVALID
    }


    pub fn valid_count(&self) -> usize {
        self.valid_numbers.iter().count()
    }
    pub fn parse_number(&self, cell: &Cell) -> usize {
        let mut num: Vec<u32> = vec![];

        for col in (0..cell.n).rev() {
            let temp: Cell = self.raw_matrix[cell.m][col].clone();
            if temp.kind == NUMBER {
                num.push(temp.number);
            } else {
                break;
            }
        }

        num.reverse();
        num.push(cell.number);

        for col in (cell.n..self.n) {
            if col == cell.n {
                continue;
            }
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
                println!("number : {}", num);
                return num;
            }
            _ => {}
        }
        0
    }

    pub fn process(&mut self) {
        let linear_matrix = self.linear_matrix.clone();
        for cell in &linear_matrix {
            let result = self.evaluate(&cell);
            if result == VALID {
                let mut _cell: Cell = cell.clone();
                _cell.valid = result;
                self.valid_numbers.push(_cell);
                self.raw_matrix[cell.m][cell.n].valid = VALID;
            }
        }
    }

    pub fn process_raw(&mut self) {
        let _matrix = self.raw_matrix.clone();
        let mut valid_numbers_final: Vec<u32> = vec![];
        for (i, raw) in _matrix.into_iter().enumerate() {
            let mut valid_numbers_set: HashSet<u32> = HashSet::new();
            for (j, cell) in raw.into_iter().enumerate() {
                let result = self.evaluate(&cell);
                if result == VALID {
                    let mut _cell: Cell = cell.clone();
                    let number = self.parse_number(&_cell);
                    valid_numbers_set.insert(number as u32);
                    // extra
                    _cell.valid = result;
                    self.valid_numbers.push(_cell);
                    self.raw_matrix[cell.m][cell.n].valid = VALID;
                }
            }
            let valid_number_raw_count = valid_numbers_set.clone().into_iter().count();
            println!("valid number raw count: {}", valid_number_raw_count);
            for num in &valid_numbers_set {
                valid_numbers_final.push(*num);
            }
        }

        for valid_number in &valid_numbers_final {
            println!("Valid number: {}", valid_number)
        }

        let final_sum: u32 = valid_numbers_final.iter().sum();
        println!("Grand sum : {}", final_sum);
    }
}

fn main() {
    println!("<Day 02>");
    let file_contents = fs::read_to_string("data/003_data.txt").expect("ok");

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

    println!("completed matrix formation!");
    engine.process_raw();
}
