// @generated automatically by Diesel CLI.

diesel::table! {
    address (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        state -> Varchar,
        #[max_length = 50]
        city -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 20]
        zip_code -> Varchar,
    }
}

diesel::table! {
    alternative_phone (user_id, area_code, number) {
        user_id -> Int4,
        #[max_length = 5]
        area_code -> Varchar,
        #[max_length = 15]
        number -> Varchar,
        #[max_length = 10]
        extension -> Nullable<Varchar>,
    }
}

diesel::table! {
    buyer_reputation (user_id) {
        user_id -> Int4,
        canceled_transactions -> Nullable<Int4>,
        #[max_length = 50]
        transactions_period -> Nullable<Varchar>,
        transactions_total -> Nullable<Int4>,
        transactions_completed -> Nullable<Int4>,
        transactions_canceled_total -> Nullable<Int4>,
        transactions_canceled_paid -> Nullable<Int4>,
        transactions_unrated_total -> Nullable<Int4>,
        transactions_unrated_paid -> Nullable<Int4>,
        transactions_not_yet_rated_total -> Nullable<Int4>,
        transactions_not_yet_rated_paid -> Nullable<Int4>,
        transactions_not_yet_rated_units -> Nullable<Int4>,
    }
}

diesel::table! {
    credit (user_id) {
        user_id -> Int4,
        consumed -> Int4,
        #[max_length = 50]
        credit_level_id -> Varchar,
    }
}

diesel::table! {
    identification (user_id) {
        user_id -> Int4,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 50]
        number -> Varchar,
    }
}

diesel::table! {
    phone (user_id) {
        user_id -> Int4,
        #[max_length = 5]
        area_code -> Varchar,
        #[max_length = 15]
        number -> Varchar,
        #[max_length = 10]
        extension -> Nullable<Varchar>,
        verified -> Bool,
    }
}

diesel::table! {
    seller_reputation (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        level_id -> Nullable<Varchar>,
        #[max_length = 50]
        power_seller_status -> Nullable<Varchar>,
        #[max_length = 50]
        transactions_period -> Nullable<Varchar>,
        transactions_total -> Nullable<Int4>,
        transactions_completed -> Nullable<Int4>,
        transactions_canceled -> Nullable<Int4>,
        ratings_positive -> Nullable<Int4>,
        ratings_negative -> Nullable<Int4>,
        ratings_neutral -> Nullable<Int4>,
    }
}

diesel::table! {
    status (user_id) {
        user_id -> Int4,
        list_allow -> Bool,
        list_codes -> Nullable<Array<Nullable<Text>>>,
        list_immediate_payment_required -> Bool,
        list_immediate_payment_reasons -> Nullable<Array<Nullable<Text>>>,
        buy_allow -> Bool,
        buy_codes -> Nullable<Array<Nullable<Text>>>,
        buy_immediate_payment_required -> Bool,
        buy_immediate_payment_reasons -> Nullable<Array<Nullable<Text>>>,
        sell_allow -> Bool,
        sell_codes -> Nullable<Array<Nullable<Text>>>,
        sell_immediate_payment_required -> Bool,
        sell_immediate_payment_reasons -> Nullable<Array<Nullable<Text>>>,
        billing_allow -> Bool,
        billing_codes -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    user_tags (user_id, tag_id) {
        user_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        nickname -> Varchar,
        registration_date -> Timestamptz,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 2]
        country_id -> Bpchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        user_type -> Varchar,
        points -> Int4,
        #[max_length = 50]
        site_id -> Varchar,
        #[max_length = 255]
        permalink -> Varchar,
        #[max_length = 50]
        seller_experience -> Nullable<Varchar>,
        #[max_length = 50]
        status_site_status -> Varchar,
        pago_tc_accepted -> Bool,
        #[max_length = 50]
        pago_account_type -> Nullable<Varchar>,
        #[max_length = 50]
        delivery -> Nullable<Varchar>,
        immediate_payment -> Bool,
        confirmed_email -> Bool,
        #[max_length = 255]
        required_action -> Nullable<Varchar>,
    }
}

diesel::joinable!(address -> users (user_id));
diesel::joinable!(alternative_phone -> users (user_id));
diesel::joinable!(buyer_reputation -> users (user_id));
diesel::joinable!(credit -> users (user_id));
diesel::joinable!(identification -> users (user_id));
diesel::joinable!(phone -> users (user_id));
diesel::joinable!(seller_reputation -> users (user_id));
diesel::joinable!(status -> users (user_id));
diesel::joinable!(user_tags -> tags (tag_id));
diesel::joinable!(user_tags -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    alternative_phone,
    buyer_reputation,
    credit,
    identification,
    phone,
    seller_reputation,
    status,
    tags,
    user_tags,
    users,
);
