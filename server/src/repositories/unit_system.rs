use crate::{
    models::unit_system::UnitSystem,
    repositories::{PostgresRepository, Repository},
};

pub trait UnitSystemRepository: Repository<UnitSystem> {}

impl UnitSystemRepository for PostgresRepository<UnitSystem> {}
