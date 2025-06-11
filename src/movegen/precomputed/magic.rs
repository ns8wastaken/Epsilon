pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub attacks: &'static [u64],
}
