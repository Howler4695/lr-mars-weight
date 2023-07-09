use std::io;

fn main() {
    println!("Enter weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let userWeight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", input);
    let mars_weight = calculate_our_weight_on_mars(userWeight);
    println!("Weight on Mars: {}", mars_weight);
}

fn calculate_our_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
