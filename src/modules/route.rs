use actix_web::web::ServiceConfig;

use crate::modules::*;

/// 配置路由
///
/// # 参数
///
/// * `config` - 一个可变的 ServiceConfig 引用，用于配置服务
pub fn router(config: &mut ServiceConfig) {
    // 配置认证模块的路由
    auth::router(config);
    // 配置用户模块的路由
    user::router(config);
}
