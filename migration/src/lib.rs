pub use sea_orm_migration::prelude::*;

mod m20240821_141459_create_post;
mod m20240821_141507_create_profile;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240821_141459_create_post::Migration),
            Box::new(m20240821_141507_create_profile::Migration),
        ]
    }
}
