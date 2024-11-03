use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::result::Error;
use crate::modules::products::model::domain::product::Product;
use crate::modules::products::model::domain::product_filter::ProductFilter;
use crate::db::paginated::{Page, Paginate};

impl Product {
    pub fn find_by_id(conn: &mut PgConnection, id: i32) -> Result<Product, Error> {
        use crate::schema::products::dsl::*;

        match products
            .select(Product::as_select())
            .find(id)
            .first(conn)
            .optional() {
            Ok(Some(product)) => Ok(product),
            Ok(None) => Err(Error::NotFound),
            _ => {
                Err(Error::NotFound)
            }
        }
    }
    pub fn filter(conn: &mut PgConnection,filters_dto: ProductFilter) -> Page<Product> {
        use crate::schema::products::dsl::*;

        let limit = match filters_dto.limit {
            Some(limit) => if limit <= 0 { 10 } else { limit },
            None => 100,
        };

        let page = match &filters_dto.page {
            Some(page_req) => {
                if page_req <= &0 { 1 } else {
                    *page_req
                }
            },
            None => 0
        };

        let title_filter = format!("%{}%", filters_dto.title.unwrap_or("".to_string()));
        let mlb_id_filter = format!("%{}%", filters_dto.mlb_id.unwrap_or("".to_string()));

        let results = products
            .filter(title.like(&title_filter))
            .filter(mlb_id.like(&mlb_id_filter))
            .select(Product::as_select())
            .paginate(page, limit)
            .load_page(conn)
            .expect("error");

        println!("{:?}", results);
        results
    }
}