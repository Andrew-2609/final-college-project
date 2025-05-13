use diesel::{PgConnection, r2d2::ConnectionManager};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(db_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB Pool")
}
