mod content;
use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("dan"),
    };
    let podcast = Media::Podcast(10);
    let movie = Media::Movie {
        title: String::from("bobs adventures"),
        director: String::from("bob"),
    };
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);
    catalog.add(movie);

    match catalog.get_by_index(40) {
        Some(value) => {
            println!("Item: {:#?}", value)
        }
        None => {
            println!("No item at that index.")
        }
    }
}
