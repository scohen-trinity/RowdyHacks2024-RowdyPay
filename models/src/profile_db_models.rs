pub struct GetProfileDB {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
    pub groups: Option<Vec<i32>>,
    pub payments: Option<Vec<i32>>,
}