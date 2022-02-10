pub struct Droid {
    pub id: i32,
    pub name: String,
}

impl Droid {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Todo {
    pub title: String,
}
