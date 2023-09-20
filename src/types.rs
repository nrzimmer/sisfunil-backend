pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;
pub type WDPool = actix_web::web::Data<crate::database::mysql::DBPool>;
