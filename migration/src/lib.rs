pub use sea_orm_migration::prelude::*;

mod m20251022_115559_create_auth_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20251022_115559_create_auth_tables::Migration)]
    }
}
