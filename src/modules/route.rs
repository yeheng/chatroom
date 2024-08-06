use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware;

use crate::modules::auth::controller::*;
use crate::modules::user::controller::*;

pub fn router(config: &mut ServiceConfig) {
    config.service(sign_up)
        .service(login);
    config.service(
        web::scope("/api")
            .service(
                web::scope("/user")
                    .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
                    .service(get_user_info)
                    .service(get_current_user)
                    .service(update_user)
            )

            // .service(
            //     web::scope("/profiles")
            //         .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
            //         .service(profile::follow_user)
            //         .service(profile::unfollow_user)
            // )
    );
}
