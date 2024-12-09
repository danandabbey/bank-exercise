#[derive(Debug)]
enum Media{
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
}

impl Media {
    fn print(&self){
        print!("{:#?}",self)
    }

    fn description(&self)->String{
        match self{
            Media::Book{title,author} =>{
                format!("Book: {},{}",title,author)
            }
            Media::Movie{title,director} =>{
                format!("Movie: {},{}",title,director)
            }
            Media::Audiobook{title} =>{
                format!("Audiobook: {}",title)
            }
        }
    } 
}
#[derive(Debug)]
struct Catalog{
    items: Vec<Media>,
}

impl Catalog{
    fn new() -> Self{
        Catalog{ items: vec![]}
    }

    fn add(&mut self, media: Media){
        self.items.push(media);
    }
}

fn main() {
    let audiobook = Media::Audiobook { title: String::from("dan") };
    println!("{}",audiobook.description())
}
