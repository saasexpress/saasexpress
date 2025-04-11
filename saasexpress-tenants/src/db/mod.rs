use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::OnceLock;

pub mod activity_repo;
pub mod service_repo;
pub mod tenant_repo;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create DB connection pool");

    // Run migrations
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}

pub fn get_pool() -> &'static DbPool {
    // file::memory:?cache=shared   ./sqlite.db

    DB_POOL.get_or_init(|| init_pool("./sqlite.db"))
}

pub fn get_conn() -> DbConnection {
    get_pool().get().expect("Failed to get DB connection")
}

pub fn generate_random_id(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
