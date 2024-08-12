// Import necessary dependencies and modules

use rbatis::executor::RBatisRef;
use rbatis::rbdc;
use rbatis::rbdc::db::ExecResult;
use rbs::to_value;
use redis::{Commands, Connection};

use crate::models::SysUser;
use crate::modules::user::model::NewUser;
use crate::{db, redis_conn};

// Define the service struct
pub struct UserService {}

// Implement methods for the service struct
impl UserService {
    pub async fn select_user_by_uid(uid: u32) -> Result<Option<SysUser>, rbdc::Error> {
        let key = format!("user:id:{}", uid);
        let mut conn: Connection = redis_conn!();
        match conn.get::<String, String>(key.clone()) {
            Ok(user) => Ok(Some(serde_json::from_str(&user).unwrap())),
            Err(_) => {
                let mut user = db!()
                    .query_decode(
                        r#"SELECT * FROM "sys_user" WHERE user_id = ?"#,
                        vec![to_value!(uid)],
                    )
                    .await;

                let s = user.as_mut();
                if let Some(u) = s.unwrap() {
                    conn.set::<String, String, String>(
                        key.clone(),
                        serde_json::to_string(u).unwrap(),
                    )
                    .unwrap();
                }

                user
            }
        }
    }

    pub async fn select_user_by_username(username: &str) -> Result<Option<SysUser>, rbdc::Error> {
        let key = format!("user:username:{}", username);
        let mut conn: Connection = redis_conn!();
        match conn.get::<String, String>(key.clone()) {
            Ok(user) => Ok(Some(serde_json::from_str(&user).unwrap())),
            Err(_) => {
                let mut user = db!()
                    .query_decode(
                        r#"SELECT * FROM "sys_user" WHERE user_name = ?"#,
                        vec![to_value!(username)],
                    )
                    .await;

                let s = user.as_mut();
                if let Some(u) = s.unwrap() {
                    conn.set::<String, String, String>(
                        key.clone(),
                        serde_json::to_string(u).unwrap(),
                    )
                    .unwrap();
                }
                user
            }
        }
    }

    pub async fn is_uname_already_exists(uname: &str) -> Result<bool, rbatis::Error> {
        let result: u32 = db!()
            .query_decode(
                r#"SELECT count(1) FROM "sys_user" WHERE username = ?"#,
                vec![to_value!(uname)],
            )
            .await?;

        Ok(result == 1)
    }

    pub async fn insert_new_user(u: &NewUser) -> Result<u64, rbatis::Error> {
        let exec_result = db!()
            .exec(
                r#"INSERT INTO "sys_user" (email, username, password) VALUES (?, ?, ?);"#,
                vec![
                    to_value!(u.email.as_str()),
                    to_value!(u.username.as_str()),
                    to_value!(u.password.as_str()),
                ],
            )
            .await?;

        Ok(exec_result.rows_affected)
    }

    pub async fn update_user(u: &SysUser) -> Result<ExecResult, rbatis::Error> {
        let exec_result = db!()
            .exec(
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
            )
            .await?;

        Ok(exec_result)
    }
}
