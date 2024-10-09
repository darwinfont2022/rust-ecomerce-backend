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
    attribute_combinations (product_id, combination_id) {
        product_id -> Int4,
        #[max_length = 50]
        combination_id -> Varchar,
        #[max_length = 255]
        combination_name -> Nullable<Varchar>,
        #[max_length = 50]
        combination_value_id -> Nullable<Varchar>,
        #[max_length = 255]
        combination_value_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    attributes (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        attribute_id -> Nullable<Varchar>,
        #[max_length = 255]
        attribute_name -> Nullable<Varchar>,
        #[max_length = 50]
        value_id -> Nullable<Varchar>,
        #[max_length = 255]
        value_name -> Nullable<Varchar>,
        #[max_length = 50]
        attribute_group_id -> Nullable<Varchar>,
        #[max_length = 255]
        attribute_group_name -> Nullable<Varchar>,
        #[max_length = 20]
        value_type -> Nullable<Varchar>,
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
    descriptions (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        description_id -> Nullable<Varchar>,
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
    pictures (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        picture_id -> Nullable<Varchar>,
        url -> Nullable<Text>,
        secure_url -> Nullable<Text>,
        #[max_length = 20]
        size -> Nullable<Varchar>,
        #[max_length = 20]
        max_size -> Nullable<Varchar>,
        #[max_length = 20]
        quality -> Nullable<Varchar>,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        #[max_length = 20]
        mlb_id -> Varchar,
        #[max_length = 10]
        site_id -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        seller_id -> Nullable<Int8>,
        #[max_length = 20]
        category_id -> Nullable<Varchar>,
        official_store_id -> Nullable<Int4>,
        price -> Nullable<Numeric>,
        base_price -> Nullable<Numeric>,
        original_price -> Nullable<Numeric>,
        #[max_length = 3]
        currency_id -> Nullable<Varchar>,
        initial_quantity -> Nullable<Int4>,
        available_quantity -> Nullable<Int4>,
        sold_quantity -> Nullable<Int4>,
        #[max_length = 20]
        buying_mode -> Nullable<Varchar>,
        #[max_length = 20]
        listing_type_id -> Nullable<Varchar>,
        start_time -> Nullable<Timestamptz>,
        stop_time -> Nullable<Timestamptz>,
        #[max_length = 20]
        condition -> Nullable<Varchar>,
        permalink -> Nullable<Text>,
        #[max_length = 50]
        thumbnail_id -> Nullable<Varchar>,
        thumbnail -> Nullable<Text>,
        #[max_length = 20]
        international_delivery_mode -> Nullable<Varchar>,
        #[max_length = 50]
        listing_source -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 50]
        warranty -> Nullable<Varchar>,
        #[max_length = 20]
        catalog_product_id -> Nullable<Varchar>,
        #[max_length = 50]
        domain_id -> Nullable<Varchar>,
        #[max_length = 20]
        parent_item_id -> Nullable<Varchar>,
        automatic_relist -> Nullable<Bool>,
        date_created -> Nullable<Timestamptz>,
        last_updated -> Nullable<Timestamptz>,
        health -> Nullable<Int4>,
        catalog_listing -> Nullable<Bool>,
    }
}

diesel::table! {
    sale_terms (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        term_id -> Nullable<Varchar>,
        #[max_length = 255]
        term_name -> Nullable<Varchar>,
        #[max_length = 50]
        term_value_id -> Nullable<Varchar>,
        #[max_length = 255]
        term_value_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    seller_address (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        city_id -> Nullable<Varchar>,
        #[max_length = 255]
        city_name -> Nullable<Varchar>,
        #[max_length = 50]
        state_id -> Nullable<Varchar>,
        #[max_length = 255]
        state_name -> Nullable<Varchar>,
        #[max_length = 50]
        country_id -> Nullable<Varchar>,
        #[max_length = 255]
        country_name -> Nullable<Varchar>,
        #[max_length = 50]
        neighborhood_id -> Nullable<Varchar>,
        #[max_length = 255]
        neighborhood_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    seller_payment_methods (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        method_id -> Nullable<Varchar>,
        #[max_length = 255]
        method_name -> Nullable<Varchar>,
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
    shipping (product_id) {
        product_id -> Int4,
        #[max_length = 20]
        mode -> Nullable<Varchar>,
        free_shipping -> Nullable<Bool>,
        #[max_length = 20]
        logistic_type -> Nullable<Varchar>,
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

diesel::table! {
    variations (product_id) {
        product_id -> Int4,
        variation_id -> Nullable<Int8>,
        price -> Nullable<Numeric>,
        available_quantity -> Nullable<Int4>,
        sold_quantity -> Nullable<Int4>,
        #[max_length = 20]
        catalog_product_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(address -> users (user_id));
diesel::joinable!(alternative_phone -> users (user_id));
diesel::joinable!(attribute_combinations -> variations (product_id));
diesel::joinable!(attributes -> products (product_id));
diesel::joinable!(buyer_reputation -> users (user_id));
diesel::joinable!(credit -> users (user_id));
diesel::joinable!(descriptions -> products (product_id));
diesel::joinable!(identification -> users (user_id));
diesel::joinable!(phone -> users (user_id));
diesel::joinable!(pictures -> products (product_id));
diesel::joinable!(sale_terms -> products (product_id));
diesel::joinable!(seller_address -> products (product_id));
diesel::joinable!(seller_payment_methods -> products (product_id));
diesel::joinable!(seller_reputation -> users (user_id));
diesel::joinable!(shipping -> products (product_id));
diesel::joinable!(status -> users (user_id));
diesel::joinable!(user_tags -> tags (tag_id));
diesel::joinable!(user_tags -> users (user_id));
diesel::joinable!(variations -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    alternative_phone,
    attribute_combinations,
    attributes,
    buyer_reputation,
    credit,
    descriptions,
    identification,
    phone,
    pictures,
    products,
    sale_terms,
    seller_address,
    seller_payment_methods,
    seller_reputation,
    shipping,
    status,
    tags,
    user_tags,
    users,
    variations,
);
