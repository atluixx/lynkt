use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};

use crate::structs::DatabasePool;

pub fn connect(database_url: &str) -> DatabasePool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(75)
        .build(manager)
        .expect("pool couldn't be created")
}
