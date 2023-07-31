use f_messages::{Exchange, InstrumentType, Lvl};
fn main() {
    let l1 = Lvl {
        ask_p: 1.0,
        bid_p: 2.0,
        ask_v: 3.0,
        bid_v: 4.0,
        ts: 45,
        inst_type: InstrumentType::Spot,
        exch: Exchange::Okex,
    };
    println!("Hello, world! {}", l1.ask_p);
}
