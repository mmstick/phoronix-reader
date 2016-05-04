use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;

/// A Phoronix Article contains a title, link, summary and details of when the article was posted and how many comments
/// have been posted on that article.
pub struct Article {
    pub title:   String,
    pub link:    String,
    pub details: String,
    pub summary: String,
}

impl Article {
    /// Obtains all of the articles from the front page of Phoronix.
    pub fn get_articles(html: &str) -> Vec<Article> {
        Document::from(html).find(Name("article")).iter()
            .map(|node| Article::new(&node)).collect()
    }

    #[inline]
    /// Creates an article structure from the provided HTML node.
    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).first().unwrap();
        let mut link = String::from(header.attr("href").unwrap());
        if link.starts_with('/') { assert_eq!(link.remove(0), '/'); }
        let mut details = node.find(Class("details")).first().unwrap().text();
        details = details.replace("Add A Comment", "0 Comments");
        let summary = node.find(Name("p")).first().unwrap().text();
        Article { title: header.text(), link: link, details: details, summary: summary }
    }
}
