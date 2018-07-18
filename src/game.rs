use board;

#[derive(Clone, Debug)]
pub struct Game {
    pub month: usize,
    pub score_oya: usize,
    pub score_ko: usize,
    pub oya: Player,
}

#[derive(Clone, Debug)]
pub enum Player {
    Human,
    Cpu,
}
