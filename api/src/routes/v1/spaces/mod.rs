use actix_web::Scope;

mod create;
mod delete;
mod get;
mod helper;
mod update;

pub fn spaces_scope() -> Scope {
    Scope::new("/spaces")
        .service(create::create_space)
        .service(get::get_space)
        .service(get::get_spaces_for_org)
        .service(delete::delete_space)
        .service(update::update_space)
}
