use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph {
    name:String
} 


#[derive(Serialize,Deserialize)]
struct Article {
    article : String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn main(){
    let article: Article = Article{
        article:String::from("How to work with json in rust"),
        author:String::from("Tushar"),
        paragraph: vec![
            Paragraph{
                name:String::from("This is the first paragraph")
            },
            Paragraph{
                name:String::from("This is the second paragraph")
            },
            Paragraph{
                name:String::from("This is the third paragraph")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The JSON is: {}", json);
}