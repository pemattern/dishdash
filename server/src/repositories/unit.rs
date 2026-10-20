use crate::{
    models::unit::Unit,
    repositories::{PostgresRepository, Repository},
};

pub trait UnitRepository: Repository<Unit> {}

impl UnitRepository for PostgresRepository<Unit> {}
