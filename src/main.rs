use rust_week_2_exercises::parse_satoshis;

fn main() {
    let data = "0x1234567812345678";

    let hex = parse_satoshis(data);
    println!("{:?}", hex);
}