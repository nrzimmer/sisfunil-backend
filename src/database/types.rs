pub type DBPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;
pub type DBManager = diesel::r2d2::ConnectionManager::<diesel::MysqlConnection>;