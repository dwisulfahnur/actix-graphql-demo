use r2d2_mysql::mysql::{Conn, Error, Opts, OptsBuilder};
use r2d2_mysql::{MysqlConnectionManager};
use r2d2;

pub type Pool = r2d2::Pool<MysqlConnectionManager>;

pub fn get_db_pool() -> Pool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    r2d2::Pool::new(manager).expect("Failed to create DB Pool")
}
