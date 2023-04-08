pub struct DB{
    pub url: &'static str
}

impl DB{
    pub fn url()->Self{
        DB{url:"postgres://bdfvrmvi:fHs9BvUfWLp7bDSDKIb_j_GvcKCIGD5Z@mouse.db.elephantsql.com/bdfvrmvi"}
    }
}