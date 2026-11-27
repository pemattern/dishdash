use lib::Payload;

use crate::{
    models::UnitModel,
    requests::constants::{PhysicalQuantityCode, UnitCode, UnitSystemCode},
};

#[allow(dead_code)]
#[derive(Payload)]
#[payload(read_only)]
struct UnitPayload {
    pub code: UnitCode,
    pub physical_quantity_code: PhysicalQuantityCode,
    pub unit_system_code: UnitSystemCode,
    pub symbol: String,
    pub si_factor: f64,
}

impl From<UnitModel> for UnitResponse {
    fn from(value: UnitModel) -> Self {
        Self {
            code: value.code,
            physical_quantity_code: value.physical_quantity_code,
            unit_system_code: value.unit_system_code,
            symbol: value.symbol,
            si_factor: value.si_factor,
        }
    }
}
