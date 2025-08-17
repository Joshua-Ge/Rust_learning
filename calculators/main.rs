use std::io;

struct SquareRoot {
    x: f64,
    n: f64,
    answer: f64,
    reps: u64
}

impl SquareRoot {
    fn new() -> Self {
        SquareRoot {
            x: 0.0,
            n: 0.0,
            answer: 0.0,
            reps: 0
        }
    }
}

impl SquareRoot {
    fn get_input(&mut self) {
        let mut input: String = String::new();

        println!("What number do you want to square root?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        self.n = input.trim().parse::<f64>().expect("Wrong convert value Current is f64");
        input.clear();

        self.x = self.n / 2.0;

        println!("How accuret do you want this to be? (How many reps will occure)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        self.reps = input.trim().parse::<u64>().expect("Wrong convert value Current is u64");
    }

    fn calculate(&mut self) {
        self.answer = 0.5 * (self.x + (self.n / self.x));
        self.x = self.answer;
    }

    fn itterate(&mut self) {
        for _ in 0..self.reps {
            self.calculate();
        }
    }
}

fn main() {
    loop {
        let mut cont: String = String::new();

        let mut calc = SquareRoot::new();
        calc.get_input();
        calc.itterate();

        println!("{}", calc.answer);
        
        println!("Would you like to continue");

        io::stdin()
            .read_line(&mut cont)
            .expect("Failed to read input");

        if cont.trim() != "y" {
            break;
        }
        
    }
}