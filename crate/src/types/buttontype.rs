/// A variant type denoting the purpose of a button
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonType {
    WaterGun,
    AcidTower,
    SodaMaker,
    Upgrade,
    Delete,
    Other,
}
