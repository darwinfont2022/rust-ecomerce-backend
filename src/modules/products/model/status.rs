use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Status {
    pub user_id: i32,
    pub list_allow: bool,
    pub list_codes: Option<Vec<Option<String>>>,
    pub list_immediate_payment_required: bool,
    pub list_immediate_payment_reasons: Option<Vec<Option<String>>>,
    pub buy_allow: bool,
    pub buy_codes: Option<Vec<Option<String>>>,
    pub buy_immediate_payment_required: bool,
    pub buy_immediate_payment_reasons: Option<Vec<Option<String>>>,
    pub sell_allow: bool,
    pub sell_codes: Option<Vec<Option<String>>>,
    pub sell_immediate_payment_required: bool,
    pub sell_immediate_payment_reasons: Option<Vec<Option<String>>>,
    pub billing_allow: bool,
    pub billing_codes: Option<Vec<Option<String>>>,
}