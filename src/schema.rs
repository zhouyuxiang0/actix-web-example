table! {
    user (id) {
        id -> Integer,
        username -> Nullable<Varchar>,
        password -> Varchar,
        salt -> Varchar,
        realname -> Nullable<Varchar>,
        cellphone -> Varchar,
        enable -> Nullable<Bool>,
        login_time -> Nullable<Datetime>,
        create_time -> Nullable<Datetime>,
        update_time -> Nullable<Datetime>,
    }
}
