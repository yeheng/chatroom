// Import necessary dependencies and modules

use rbatis::executor::RBatisRef;
use rbatis::rbdc;
use rbs::to_value;

use crate::middleware::datasource::DB;
use crate::model::user::User;
use crate::models::SysUser;
use crate::modules::user::model::{NewUser};

// Define the service struct
pub struct UserService {
    // Define any necessary fields here
}

// Implement methods for the service struct
impl UserService {
    pub async fn select_user_by_uid(uid: u32) -> Result<Option<User>, rbatis::Error> {
        let rb = DB.rb.rb_ref();
        rb.query_decode(
            r#"SELECT * FROM "sys_user" WHERE user_id = ?"#,
            vec![to_value!(uid)],
        ).await
    }

    pub async fn select_user_by_email(email: &str) -> Result<Option<SysUser>, rbdc::Error> {
        let rb = DB.rb.rb_ref();
        rb.query_decode(
            r#"SELECT * FROM "sys_user" WHERE email = ?"#,
            vec![to_value!(email)],
        ).await
    }

    pub async fn is_uname_already_exists(uname: &str) -> Result<bool, rbatis::Error> {
        let rb = DB.rb.rb_ref();
        let result: u32 = rb
            .query_decode(
                r#"SELECT count(1) FROM "sys_user" WHERE username = ?"#,
                vec![to_value!(uname)],
            )
            .await?;

        Ok(result == 1)
    }

    pub async fn insert_new_user(u: &NewUser) -> Result<u64, rbatis::Error> {
        let rb = DB.rb.rb_ref();
        let exec_result = rb.exec(
            r#"INSERT INTO "sys_user" (email, username, password) VALUES (?, ?, ?);"#,
            vec![to_value!(u.email.as_str()), to_value!(u.username.as_str()), to_value!(u.password.as_str())],
        ).await?;

        Ok(exec_result.rows_affected)
    }

    pub async fn update_user(u: &SysUser) -> Result<i64, rbatis::Error> {
        let rb = DB.rb.rb_ref();

        let exec_result = rb.exec(
            r#"
        UPDATE "sys_user"
        SET email = ?, user_name = ?, password = ?, nick_name = ?
        WHERE uid = ?;
        "#,
            vec![
                to_value!(&u.email),
                to_value!(&u.user_name),
                to_value!(&u.password),
                to_value!(&u.nick_name),
                to_value!(u.user_id),
            ],
        ).await?;

        Ok(exec_result.rows_affected as i64)
    }
}
