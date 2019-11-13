/// Different upgrade variants of a Water Gun
pub enum WaterGun {
    Basic,
    SuperSoaker,
    ExtremeSoaker,
}
/// Different upgrade variants of an Acid Tower
pub enum AcidTower {
    Basic,
    Radioactive,
}
/// Different upgrade variants of a Soda Maker
pub enum SodaMaker {
    Basic,
    SparklingWater,
    RootBeer,
}

/// Represents the current type of Tower
pub enum TowerType {
    WaterGun(WaterGun),
    AcidTower(AcidTower),
    SodaMaker(SodaMaker),
}
