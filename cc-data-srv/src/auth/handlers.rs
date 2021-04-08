use warp::{Filter, Reply, reject, reply, Rejection};
use std::convert::Infallible;
use crate::auth::error::{WebResult, handle_rejection};
use crate::auth::jwt_auth::{Role, create_jwt, with_auth};
use crate::auth::error::Error::WrongCredentialsError;
use crate::auth::user::{Users, init_users};
use crate::acl::permissions::SecurityManager;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

fn with_security_manager(secmgr: SecurityManager) -> impl Filter<Extract = (SecurityManager,), Error = Infallible> + Clone {
    warp::any().map(move || secmgr.clone())
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = create_jwt(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

// ...
pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

pub fn login_routes() -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);
    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(handle_rejection);

    routes
}

