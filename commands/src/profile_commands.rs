use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProfileCommand {
    pub id: u64,
}
