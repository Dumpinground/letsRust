fn main() {
    let mut speed = 1.0_f32;
    let damp = 0.92;

    while speed > 0.0001 {
        speed *= damp;
        println!("{speed}");
    }

    speed = 0.;
    println!("{speed}");
}
