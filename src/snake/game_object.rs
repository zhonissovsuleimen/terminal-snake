#[derive(Clone, Copy)]
pub enum GameObject {
    Snake(u16, u16),
    Food(u16, u16),
}
