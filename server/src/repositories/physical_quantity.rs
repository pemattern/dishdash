use crate::{
    models::physical_quantity::PhysicalQuantity,
    repositories::{PostgresRepository, Repository},
};

pub trait PhysicalQuantityRepository: Repository<PhysicalQuantity> {}

impl PhysicalQuantityRepository for PostgresRepository<PhysicalQuantity> {}
