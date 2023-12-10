use std::fs;
use crate::Color::{BLUE, GREEN, RED};

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl Color {
    pub fn new(value: &str) -> Result<Color, ()> {
        let colour = match value {
            "red" => { Ok(RED) }
            "green" => { Ok(GREEN) }
            "blue" => { Ok(BLUE) }
            _ => { Err(()) }
        };
        return colour;
    }
    pub fn max(&self) -> u32 {
        match self {
            RED => { 12 }
            GREEN => { 13 }
            BLUE => { 14 }
        }
    }
}

struct Primitive {
    kind: Color,
    value: u32,
}


struct Game {
    id: u32,
    input: String,
    primitive_super_set: Vec<Vec<Primitive>>,
    valid: bool,
}


impl Game {
    pub fn new(input: String) -> Self {
        Game {
            input,
            id: 0,
            primitive_super_set: vec![],
            valid: false,
        }
    }

    fn get_primitive(&self, primitive: &str) -> Primitive {
        let primitive_split: Vec<&str> = primitive.split(" ").collect();
        let kind = match Color::new(primitive_split[2]) {
            Ok(colour_type) => { colour_type }
            Err(_) => { panic!("failed to parse the colour!") }
        };
        let value: u32 = primitive_split[1].parse().unwrap();
        Primitive {
            kind,
            value,
        }
    }

    fn parse_game_string(&mut self) {
        let input_slice: Vec<&str> = self.input.split(":").collect();
        let game_slice: Vec<&str> = input_slice[0].split(" ").collect();
        let colour_slice: Vec<&str> = input_slice[1].split(";").collect();

        let game_id: u32 = game_slice[1].parse().unwrap();
        self.id = game_id;

        let mut primitive_super_set: Vec<Vec<Primitive>> = vec![];

        for game_set in colour_slice {
            let atomic_colours: Vec<&str> = game_set.split(",").collect();
            let mut primitive_set: Vec<Primitive> = vec![];
            for primitive in atomic_colours {
                primitive_set.push(self.get_primitive(&primitive));
            }
            primitive_super_set.push(primitive_set);
        }

        self.primitive_super_set = primitive_super_set;
    }

    pub fn validate(&mut self) {
        println!("{}", self.input);

        self.parse_game_string();

        for primitive_superset in &self.primitive_super_set {
            for primitive in primitive_superset {
                match primitive.kind {
                    RED => {
                        if primitive.value > RED.max() {
                            self.valid = false;
                            return;
                        }
                    }
                    GREEN => {
                        if primitive.value > GREEN.max() {
                            self.valid = false;
                            return;
                        }
                    }
                    BLUE => {
                        if primitive.value > BLUE.max() {
                            self.valid = false;
                            return;
                        }
                    }
                };
            }
        };
        self.valid = true;
    }

    pub fn is_valid_str(&self) -> String {
        if self.valid {
            return String::from("valid");
        }
        String::from("invalid")
    }
}

fn main() {
    println!("<Day 02>");

    let file_contents = fs::read_to_string("data/002_data.txt").expect("ok");
    let mut games: Vec<Game> = vec![];

    for game_string in file_contents.split("\n") {
        let mut game = Game::new(String::from(game_string));
        game.validate();
        games.push(game);
    }

    let sum: u32 = games.iter()
        .filter(|instance| instance.valid)
        .map(|instance| instance.id)
        .sum();

    println!("sum_of_possible_game_ids: {}", sum);
}
