#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Flags {
    Final = 1,
    Clientgen = 2,
    Nmvgen = 4,
    Extdata = 8,
    Sdsingle = 16,
}
