pub enum Side {
    /// player who goes first
    Sente,
    /// player who goes second
    Gote,
}

impl Side {
    pub const COUNT: usize = 2;
}
