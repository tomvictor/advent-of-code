enum Color {
    RED,
    BLUE,
    GREEN,
}

impl Color {
    pub fn new(value: &str) -> Result<Color, ()> {
        let colour = match value {
            "red" => { Ok(Color::RED) }
            "green" => { Ok(Color::GREEN) }
            "blue" => { Ok(Color::BLUE) }
            _ => { Err(()) }
        };
        return colour;
    }
    pub fn count(&self) -> u32 {
        let total_count = match self {
            Color::RED => { 12 }
            Color::BLUE => { 13 }
            Color::GREEN => { 14 }
        };
        total_count
    }
}

struct Primitive {
    kind: Color,
    value: u32,
}

struct Game {
    input: String,
}

impl Game {
    pub fn new(input: String) -> Game {
        Game {
            input
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

    fn get_colour_sets(self) {
        let input_slice: Vec<&str> = self.input.split(":").collect();
        let colour_split: Vec<&str> = input_slice[1].split(";").collect();

        let mut primitive_set: Vec<Primitive> = vec![];

        for game_set in colour_split {
            let atomic_colours: Vec<&str> = game_set.split(",").collect();
            for primitive in atomic_colours {
                let primitive_instance = self.get_primitive(&primitive);
                primitive_set.push(primitive_instance);
            }
        }
    }

    pub fn validate(self) -> bool {
        println!("{}", self.input);
        self.get_colour_sets();
        true
    }
}

fn main() {
    println!("<Day 02>");

    let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let game = Game::new(String::from(input_string));
    let result = game.validate();
}
