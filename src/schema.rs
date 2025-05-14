// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
    }
}

diesel::table! {
    appointments (id) {
        id -> Int4,
        patient_id -> Int4,
        appointment_at -> Timestamp,
        #[max_length = 100]
        specialty -> Varchar,
        notes -> Nullable<Text>,
        canceled -> Bool,
        canceled_at -> Nullable<Timestamp>,
        cancellation_reason -> Nullable<Text>,
    }
}

diesel::table! {
    patients (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 11]
        cpf -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        address -> Varchar,
    }
}

diesel::joinable!(appointments -> patients (patient_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    appointments,
    patients,
    users,
);
