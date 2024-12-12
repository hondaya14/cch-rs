use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SpecifiedUnit {
    pub liters: Option<f32>,
    pub litres: Option<f32>,
    pub gallons: Option<f32>,
    pub pints: Option<f32>,
}
impl SpecifiedUnit {
    pub fn validate(&self) -> bool {
        // check if only one unit is specified
        let count = [self.liters, self.litres, self.gallons, self.pints]
            .iter()
            .filter(|&&unit| unit.is_some())
            .count();
        count <= 1
    }

    pub fn get_unit_type(&self) -> MilkUnitType {
        if self.liters.is_some() {
            MilkUnitType::Liters
        } else if self.litres.is_some() {
            MilkUnitType::Litres
        } else if self.gallons.is_some() {
            MilkUnitType::Gallons
        } else {
            MilkUnitType::Pints
        }
    }
}

pub enum MilkUnitType {
    Liters,
    Litres,
    Gallons,
    Pints,
}

pub struct MilkUnits {
    pub liters: f32,
    pub gallons: f32,
    pub pints: f32,
    pub unit_type: MilkUnitType,
}
impl From<SpecifiedUnit> for MilkUnits {
    fn from(unit: SpecifiedUnit) -> Self {
        let unit_type = unit.get_unit_type();
        match unit_type {
            MilkUnitType::Liters => MilkUnits::new(unit.liters.unwrap(), MilkUnitType::Litres),
            MilkUnitType::Litres => MilkUnits::new(unit.litres.unwrap(), MilkUnitType::Litres),
            MilkUnitType::Gallons => MilkUnits::new(unit.gallons.unwrap(), MilkUnitType::Gallons),
            MilkUnitType::Pints => MilkUnits::new(unit.pints.unwrap(), MilkUnitType::Pints),
        }
    }
}

impl MilkUnits {
    const LITERS_TO_GALLONS: f32 = 0.264172;
    const LITTERS_TO_PINTS: f32 = 1.75975;
    const GALLONS_TO_LITERS: f32 = 3.78541;
    const GALLONS_TO_PINTS: f32 = 6.66139;
    const PINTS_TO_LITERS: f32 = 0.568261;
    const PINTS_TO_GALLONS: f32 = 0.150119;

    pub fn new(value: f32, unit_type: MilkUnitType) -> Self {
        // type conversion
        let liters = match unit_type {
            MilkUnitType::Liters | MilkUnitType::Litres => value,
            MilkUnitType::Gallons => value * Self::GALLONS_TO_LITERS,
            MilkUnitType::Pints => value / Self::LITTERS_TO_PINTS,
        };

        let gallons = match unit_type {
            MilkUnitType::Liters | MilkUnitType::Litres => value * Self::LITERS_TO_GALLONS,
            MilkUnitType::Gallons => value,
            MilkUnitType::Pints => value * Self::PINTS_TO_GALLONS,
        };

        let pints = match unit_type {
            MilkUnitType::Liters | MilkUnitType::Litres => value * Self::LITTERS_TO_PINTS,
            MilkUnitType::Gallons => value * Self::GALLONS_TO_PINTS,
            MilkUnitType::Pints => value,
        };

        Self {
            liters,
            gallons,
            pints,
            unit_type,
        }
    }
}