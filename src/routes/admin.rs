use actix_web::{web, HttpResponse, Responder};

use crate::models::invites::Invite;
use crate::structs::{Response, State};

pub async fn create_invite(state: web::Data<State>) -> impl Responder {
    let inv = Invite::generate(&state.database);
    return HttpResponse::Ok().content_type("application/json").body(
        Response {
            status: String::from("success"),
            data: inv.invite,
        }
        .json(),
    );
}

pub async fn get_all_invites(state: web::Data<State>) -> impl Responder {
    let invites: Vec<Invite> = Invite::get_all(&state.database);
    HttpResponse::Ok().content_type("application/json").body(
        Response {
            status: String::from("success"),
            data: invites,
        }
        .json(),
    )
}