// #[derive(juniper::GraphQLScalarValue)]
// pub struct UserId(i32);

#[derive(juniper::GraphQLObject)]
struct User {
    id: i32,
    username: String,
    password: String,
    create_time: chrono::NaiveDateTime,
    update_time: chrono::NaiveDateTime,
}
