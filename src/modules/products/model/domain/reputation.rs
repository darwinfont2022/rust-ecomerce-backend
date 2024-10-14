use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::seller_reputation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reputation{
    pub user_id: i32,
    pub level_id: Option<String>,
    pub power_seller_status: Option<String>,
    pub transactions_period: Option<String>,
    pub transactions_total: Option<i32>,
    pub transactions_completed: Option<i32>,
    pub transactions_canceled: Option<i32>,
    pub ratings_positive: Option<i32>,
    pub ratings_negative: Option<i32>,
    pub ratings_neutral: Option<i32>,
}