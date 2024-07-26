use crate::utils::Number;

pub enum Targets {
    All,
    One,
}

pub enum CardActions {
    Damage((Number, Targets)),
    ApplyVulnerable((Number, Targets)),
    ApplyWeak((Number, Targets)),
    Block(Number),
    Draw(u8),
    LoseHealth(u16), // TODO: Other things cards can do
}
