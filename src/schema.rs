table! {
    crier_user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    listing (id) {
        id -> Int4,
        seller_id -> Int4,
        title -> Varchar,
        cost -> Int4,
        currency -> Varchar,
        amount -> Nullable<Int4>,
        limited_amount -> Nullable<Bool>,
    }
}

table! {
    payer (id) {
        id -> Int4,
        crier_user_id -> Int4,
        service_customer_id -> Nullable<Varchar>,
        service_payment_source -> Varchar,
    }
}

table! {
    seller (id) {
        id -> Int4,
        crier_user_id -> Int4,
        access_token -> Varchar,
        refresh_token -> Nullable<Varchar>,
        publishable_key -> Nullable<Varchar>,
        service_id -> Varchar,
    }
}

joinable!(listing -> seller (seller_id));
joinable!(payer -> crier_user (crier_user_id));
joinable!(seller -> crier_user (crier_user_id));

allow_tables_to_appear_in_same_query!(
    crier_user,
    listing,
    payer,
    seller,
);
