// @generated automatically by Diesel CLI.

diesel::table! {
    order_detail_table (id) {
        id -> Integer,
        order_id -> Integer,
        product_id -> Integer,
        product_quantity -> Integer,
        unit_price -> Decimal,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        order_status_id -> Integer,
    }
}

diesel::table! {
    order_status_table (id) {
        id -> Integer,
        #[max_length = 50]
        status_name -> Varchar,
        #[max_length = 50]
        status_description -> Varchar,
    }
}

diesel::table! {
    order_table (id) {
        id -> Integer,
        #[max_length = 36]
        user_uuid -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    payment_methods_table (id) {
        id -> Integer,
        #[max_length = 50]
        payment_method_name -> Varchar,
        #[max_length = 255]
        payment_method_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    payment_table (id) {
        id -> Integer,
        order_id -> Integer,
        payment_amount -> Decimal,
        payment_status -> Integer,
        payment_method_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_status_table (id) {
        id -> Integer,
        #[max_length = 20]
        status_name -> Varchar,
        #[max_length = 20]
        status_description -> Varchar,
    }
}

diesel::table! {
    product_table (id) {
        id -> Integer,
        product_type_id -> Integer,
        #[max_length = 50]
        product_name -> Varchar,
        product_price -> Decimal,
        product_stock -> Integer,
        #[max_length = 255]
        product_description -> Nullable<Varchar>,
        #[max_length = 255]
        product_icon -> Nullable<Varchar>,
        product_status_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_types_table (id) {
        id -> Integer,
        #[max_length = 50]
        type_name -> Varchar,
        #[max_length = 255]
        type_icon -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles_table (id) {
        id -> Integer,
        #[max_length = 25]
        role_name -> Varchar,
        #[max_length = 50]
        role_description -> Varchar,
    }
}

diesel::table! {
    user_profile_table (user_uuid) {
        #[max_length = 36]
        user_uuid -> Varchar,
        #[max_length = 50]
        real_name -> Nullable<Varchar>,
        #[max_length = 255]
        bio -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 10]
        gender -> Nullable<Varchar>,
        age -> Nullable<Integer>,
    }
}

diesel::table! {
    user_roles_table_correlation (user_uuid, role_id) {
        #[max_length = 36]
        user_uuid -> Varchar,
        role_id -> Integer,
    }
}

diesel::table! {
    user_status_table (id) {
        id -> Integer,
        #[max_length = 20]
        status_name -> Varchar,
        #[max_length = 50]
        status_description -> Varchar,
    }
}

diesel::table! {
    users_table (uuid) {
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 25]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        user_status_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(order_detail_table -> order_status_table (order_status_id));
diesel::joinable!(order_detail_table -> order_table (order_id));
diesel::joinable!(order_detail_table -> product_table (product_id));
diesel::joinable!(order_table -> users_table (user_uuid));
diesel::joinable!(payment_table -> order_table (order_id));
diesel::joinable!(payment_table -> payment_methods_table (payment_method_id));
diesel::joinable!(product_table -> product_types_table (product_type_id));
diesel::joinable!(user_profile_table -> users_table (user_uuid));
diesel::joinable!(user_roles_table_correlation -> roles_table (role_id));
diesel::joinable!(user_roles_table_correlation -> users_table (user_uuid));
diesel::joinable!(users_table -> user_status_table (user_status_id));

diesel::allow_tables_to_appear_in_same_query!(
    order_detail_table,
    order_status_table,
    order_table,
    payment_methods_table,
    payment_table,
    product_status_table,
    product_table,
    product_types_table,
    roles_table,
    user_profile_table,
    user_roles_table_correlation,
    user_status_table,
    users_table,
);
