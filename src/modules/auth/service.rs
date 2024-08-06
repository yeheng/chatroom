// Import necessary dependencies and modules

use std::borrow::Borrow;

use rbatis::rbdc;
use rbs::to_value;

use crate::middleware::datasource::DB;

use super::model::UserTable;

// Define the service struct
pub struct AuthService {
    // Define any necessary fields here
}

// Implement methods for the service struct
impl AuthService {
    pub async fn select_user_by_email(email: &str) -> Result<Option<UserTable>, rbdc::Error> {
        let rb = DB.rb.borrow();
        rb.query_decode(
            r#"SELECT * FROM "user" WHERE email = ?"#,
            vec![to_value!(email)],
        ).await
    }

    pub async fn is_uname_already_exists(uname: &str) -> Result<bool, rbatis::Error> {
        let rb = DB.rb.borrow();
        let result: u32 = rb
            .query_decode(
                r#"SELECT count(1) FROM "user" WHERE username = ?"#,
                vec![to_value!(uname)],
            )
            .await?;

        Ok(result == 1)
    }
}
