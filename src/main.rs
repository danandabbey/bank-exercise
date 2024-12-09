#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn print(&self) {
        print!("{:#?}", self)
    }

    fn description(&self) -> String {
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
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("dan"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);

    match item {
        Some(value) => {
            println!("Item: {:#?}", value)
        }
        None => {
            println!("No item at that index.")
        }
    }
}
