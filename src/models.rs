pub struct Message {
    pub author: User,
    pub content: String,
}

pub struct User {
    pub name: String,
    pub status: Status,
}

#[derive(Default)]
pub enum Status {
    Online,
    DoNotDisturb,
    Idle,
    #[default]
    Away,
}

#[derive(Default)]
pub enum Role {
    Admin,
    Moderator,
    #[default]
    Member,
}
