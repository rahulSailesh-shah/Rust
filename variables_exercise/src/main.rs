const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!(
        "Firing {} of my {} missiles...",
        READY_AMOUNT, STARTING_MISSILES
    );

    println!("{} missiles left", missiles - ready);
}
