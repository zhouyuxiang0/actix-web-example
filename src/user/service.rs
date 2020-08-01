struct CreateUser {
    username: String,
    password: String
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}