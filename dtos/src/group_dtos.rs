use serde::Serialize;

#[derive(Serialize)]
pub struct GroupDB {
    pub group_id: i32,
    pub group_name: String,
    pub users: Option<Vec<i32>>,
    pub img: Option<String>,
}

#[derive(Serialize)]
pub struct PartialGroupDB {
    pub group_id: i32,
    pub group_name: String,
    pub img: Option<String>,
}

#[derive(Serialize)]
pub struct GroupUserDB {
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct ParticipantsDB {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
    pub groups: Option<Vec<i32>>,
    pub payments: Option<Vec<i32>>,
    pub date_created: i32,
}
