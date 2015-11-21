use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
use article::Article;
use linesplit;
use term;

fn get_homepage() -> String {
    let client = Client::new();
    let mut response = client.get("https://www.phoronix.com/").
        header(Connection::close()).send().unwrap(); 
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    return body;
}

// fn get_homepage_offline() -> String {
//     String::from(include_str!("phoronix.html"))
// }

pub fn print_homepage() {
    let phoronix_articles = Article::get_articles(&get_homepage());
    for article in phoronix_articles.iter().rev() {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary:");
        for line in linesplit::split_by_chars(&article.summary, 77) {
            println!(" - {}", line);
        }
        print!("\n");
    }
}

pub fn print_homepage_colored() {
    let phoronix_articles = Article::get_articles(&get_homepage());
    let mut terminal = term::stdout().unwrap();
    for article in phoronix_articles.iter().rev() {
        print!("Title:   ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.title);
        terminal.reset().unwrap();
        print!("Link:    ");
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();
        println!("https://www.phoronix.com/{}", article.link);
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

