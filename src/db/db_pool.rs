use diesel::{PgConnection};
use diesel::r2d2::{Pool, ConnectionManager};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;