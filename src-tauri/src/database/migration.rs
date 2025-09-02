use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub(super) fn run_pending_migrations(conn: &mut SqliteConnection) {
  conn
    .run_pending_migrations(MIGRATIONS)
    .unwrap();
}
