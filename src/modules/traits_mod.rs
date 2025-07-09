use diesel::prelude::MysqlConnection;

use crate::storage::mysql_connection;

pub trait DBConnectionTrait {
    fn get_connection(&self) -> Result<MysqlConnection, String> {
        let mysql_conn = mysql_connection();
        Ok(mysql_conn)
    }
}
