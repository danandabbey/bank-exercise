#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn print(&self) {
        println!("{:#?}", self)
    }

    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {},{}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {},{}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(ep) => {
                format!("Podcast: {}", ep)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
