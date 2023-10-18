use std::fmt::Display;

fn main() {
    let speed = 3;

    let sport = Sport::new(|| speed, || speed * 2 + 1);

    println!("{}", sport);
}

struct Sport<F1, F2>
where F1: Fn() -> i8, F2: Fn() -> i8
{
    walk: F1,
    run: F2
}

impl<F1, F2> Sport<F1, F2> 
where F1: Fn() -> i8, F2: Fn() -> i8
{
    fn new(walk: F1, run: F2) -> Self {
        Self { walk, run }
    }
}

impl<F1, F2> Display for Sport<F1, F2>
where F1: Fn() -> i8, F2: Fn() -> i8 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I walk {}m/s, run {}m/s", (self.walk)(), (self.run)())
    }
}