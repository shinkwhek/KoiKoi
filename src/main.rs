extern crate koikoi;
use koikoi::board;

fn main() {
    let bd = board::Board::new();
    println!(
        "{}\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}",
        bd.yama_count, bd.yama, bd.ba, bd.te_oya, bd.te_ko
    );
}
