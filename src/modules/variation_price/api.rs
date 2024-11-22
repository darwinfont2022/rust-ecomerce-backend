use crate::modules::variation_price::model::domain::variation_price::VariationPrice;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::modules::variation_price::model::dto::variation_price_dto::{VariationPriceDto, VariationPriceUpdateReq};
use crate::modules::variation_price::model::dto::variation_price_query::VariationPriceQuery;

impl VariationPrice {
    pub fn api() -> Scope {
        web::scope("/api/variation_price")
            .service(save)
            .service(load)
            .service(update)
            .service(delete)
    }
}

#[post("/{id}")]
async fn save(db_poll: web::Data<DbPool>, id: web::Path<i32>,dto: web::Json<VariationPriceDto>) -> impl Responder {
    let mut conn = db_poll.get().expect("Can't get DB connection");
    let dto = dto.into_inner();

    match VariationPrice::save(&mut conn, dto.into(), id.into_inner()) {
        Ok(price_saved) => {
            HttpResponse::Ok().json(price_saved)
        },
        Err(error) => {
            HttpResponse::BadRequest().json(error.to_string())
        }
    }
}

#[get("")]
async fn load(db_pool: web::Data<DbPool>, querys: web::Query<VariationPriceQuery>) -> impl Responder {
    let mut conn = db_pool.get().expect("Can't get DB connection");

    let detail = querys.detail.unwrap_or(false);

    if detail {
        match VariationPrice::find_all_by_variation(&mut conn, querys.variation_id) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(error) => {
                HttpResponse::BadRequest().json(error.to_string())
            }
        }
    } else {
        match VariationPrice::find_by_variation(&mut conn, querys.variation_id) {
            Ok(result) => {
                HttpResponse::Ok().json(result)
            },
            Err(e) => {
                HttpResponse::BadRequest().json(e.to_string())
            }
        }
    }
}

#[put("/{price_id}")]
async fn update(db_poll: web::Data<DbPool>, id: web::Path<i32>, dto: web::Json<VariationPriceUpdateReq>) -> impl Responder {
    let mut conn = db_poll.get().expect("Can't get DB connection");

    match VariationPrice::update(&mut conn, id.into_inner(), dto.into_inner().into()) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[delete("/{price_id}")]
async fn delete(db_poll: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let mut conn = db_poll.get().expect("Can't get DB connection");
    match VariationPrice::delete(&mut conn, id.into_inner()) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}