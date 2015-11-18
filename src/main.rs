extern crate hyper;
use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;
extern crate term;
mod linesplit;

fn main() {
    let mut terminal = term::stdout().unwrap();
    for article in Phoronix::get_articles().iter().rev() {
        print!("Title:   ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.header);
        terminal.reset().unwrap();
        print!("URL:     ");
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();
        println!("https://www.phoronix.com/{}", article.url);
        terminal.reset().unwrap();
        println!("Details: {}\nSummary:", article.details);
        for line in linesplit::split_by_chars(&article.summary, 77).iter() {
            print!(" - ");
            terminal.attr(term::Attr::Bold).unwrap();
            println!("{}", line);
            terminal.reset().unwrap();
        }
        println!("");
    }
}

struct Phoronix {
    header:  String,
    url:     String,
    details: String,
    summary: String,
}

impl Phoronix {
    // open() connects to Phoronix's main page and downloads the resulting HTML text as a String
    fn open() -> String {
        let client = Client::new();
        let mut response = client.get("https://www.phoronix.com/").
            header(Connection::close()).send().unwrap(); 
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        return body;
    }

//    open_test() opens a cached copy of Phoronix's website
//    fn open_testing() -> String {
//        String::from(include_str!("phoronix-sample.html"))
//    }

    // get_articles() collects a vector of Phoronix articles and their associated information
    fn get_articles() -> Vec<Phoronix> {
        Document::from_str(&Phoronix::open()).find(Name("article")).iter().
            map(|node| Phoronix::get_article(&node)).collect()
	}

    // get_article() returns a Phoronix struct containing article information
    fn get_article(node: &Node) -> Phoronix {
        let header = node.find(Name("a")).first().unwrap();
        let mut url = String::from(header.attr("href").unwrap());
        if url.starts_with("/") { assert_eq!(url.remove(0), '/'); }
        let summary = node.find(Name("p")).first().unwrap().text();
        let details = node.find(Class("details")).first().unwrap().text();
        Phoronix{header: header.text(), url: url, details: details, summary: summary}
    }
}

