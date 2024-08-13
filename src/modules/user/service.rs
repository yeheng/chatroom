// Import necessary dependencies and modules

use crate::model::sys_user;
use crate::modules::user::model::NewUser;
use crate::redis_conn;
use redis::{Commands, Connection};
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseBackend;
use sea_orm::DatabaseConnection;
use sea_orm::QueryResult;
use sea_orm::Statement;
use sea_orm::Value;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

// Define the service struct
pub struct UserService {}

// Implement methods for the service struct
impl UserService {
    pub async fn select_user_by_uid(
        con: &DatabaseConnection,
        uid: i64,
    ) -> Result<Option<sys_user::Model>, sea_orm::DbErr> {
        let key = format!("user:id:{}", uid);
        let mut conn: Connection = redis_conn!();
        match conn.get::<String, String>(key.clone()) {
            Ok(user) => Ok(Some(serde_json::from_str::<sys_user::Model>(&user).unwrap())),
            Err(_) => {
                let user = sys_user::Entity::find()
                    .filter(sys_user::Column::UserId.eq(uid))
                    .one(con)
                    .await?;

                if user.is_some() {
                    conn.set::<String, String, String>(
                        key.clone(),
                        serde_json::to_string(&user).unwrap(),
                    )
                    .unwrap();
                }

                Ok(user)
            }
        }
    }

    pub async fn select_user_by_username(
        con: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<sys_user::Model>, DbErr> {
        let key = format!("user:username:{}", username);
        let mut conn: Connection = redis_conn!();
        match conn.get::<String, String>(key.clone()) {
            Ok(user) => Ok(Some(serde_json::from_str::<sys_user::Model>(&user).unwrap())),
            Err(_) => {
                let user = sys_user::Entity::find()
                    .filter(sys_user::Column::UserName.eq(username))
                    .one(con)
                    .await?;

                if user.is_some() {
                    conn.set::<String, String, String>(
                        key.clone(),
                        serde_json::to_string(&user).unwrap(),
                    )
                    .unwrap();
                }
                Ok(user)
            }
        }
    }

    pub async fn is_uname_already_exists(
        con: &DatabaseConnection,
        uname: &str,
    ) -> Result<bool, DbErr> {
        let query_res: Option<QueryResult> = con
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT count(1) AS count FROM "sys_user" WHERE username = ?"#,
                [uname.into()],
            ))
            .await?;

        match query_res {
            Some(qr) => {
                let count: i64 = qr.try_get("", "count")?;
                Ok(count > 0)
            }
            None => Ok(false),
        }
    }

    pub async fn insert_new_user(con: &DatabaseConnection, u: &NewUser) -> Result<u64, DbErr> {
        let exec_result = con
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "sys_user" (email, username, password) VALUES (?, ?, ?);"#,
                [
                    u.email.as_bytes().into(),
                    u.username.as_bytes().into(),
                    u.password.as_bytes().into(),
                ],
            ))
            .await?;

        Ok(exec_result.last_insert_id())
    }

    pub async fn update_user(con: &DatabaseConnection, u: &sys_user::Model) -> Result<u64, DbErr> {
        let exec_result = con
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"
        UPDATE "sys_user"
        SET email = ?, user_name = ?, password = ?, nick_name = ?
        WHERE uid = ?;
        "#,
                [
                    Value::from(u.email.clone()),
                    Value::from(u.user_name.clone()),
                    Value::from(u.password.clone()),
                    Value::from(u.nick_name.clone()),
                    Value::from(u.user_id),
                ],
            ))
            .await?;

        Ok(exec_result.rows_affected())
    }
}
