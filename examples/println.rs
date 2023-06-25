fn main() {
    println!("{} days!", 31);
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        subject = "the quick brown fox",
        verb = "jumps over",
        object = "the lazy dog"
    );
    println!("Base 10:         {}", 69420);
    println!("Base  2(binary): {:b}", 69420);
    println!("Base  8(octal):  {:o}", 69420);
    println!("Base 16(hex):    {:x}", 69420);
    println!("Base 16(HEX):    {:X}", 69420);
}
