mod change_password;
mod cookie;
mod delete_account;
mod error;
mod login;
mod logout;
mod register;
mod routes;
mod token;
mod whoami;

pub use routes::routes;
pub use token::mw_context;

const AUTH_TOKEN: &str = "auth_token";
