#[derive(Debug)]
pub struct Information {
    pub domain: String,
    pub id: String,
    pub key: i64,
    pub name: String,
    pub encoded_name: String,
}

impl Information {
    pub fn link(&self) -> String {
        format!(
            "https://{}/d/{}/{}/DOWNLOAD",
            self.domain, self.id, self.key
        )
    }

    pub fn full_link(&self) -> String {
        format!(
            "https://{}/d/{}/{}/{}",
            self.domain, self.id, self.key, self.encoded_name
        )
    }
}
