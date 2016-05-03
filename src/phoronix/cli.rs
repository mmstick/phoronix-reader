use std::io::{self, Write};
use std::process;
use phoronix::article::Article;
use phoronix::homepage;
use linesplit;
use term;

pub fn print() {
    let stdout = &mut io::stdout();
    let phoronix_articles = Article::get_articles(&homepage::online());
    for article in phoronix_articles.iter().rev() {
        print!("Title:   {}\n", article.title);
        print!("Link:    https://www.phoronix.com/{}\n", article.link);
        print!("Details: {}\n", article.details);
        print!("Summary:\n");
        for line in linesplit::split_by_chars(&article.summary, 77) { print!(" - {}\n", line); }
        print!("\n");
    }
    let _ = stdout.flush();
    process::exit(0);
}

pub fn print_colored() {
    let stdout = &mut io::stdout();
    let phoronix_articles = Article::get_articles(&homepage::online());
    let mut terminal = term::stdout().unwrap();
    for article in phoronix_articles.iter().rev() {
        print!("Title:   ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        print!("{}\n", article.title);
        terminal.reset().unwrap();
        print!("Link:    ");
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();
        print!("https://www.phoronix.com/{}\n", article.link);
        terminal.reset().unwrap();
        print!("Details: {}\nSummary:\n", article.details);
        for line in linesplit::split_by_chars(&article.summary, 77) {
            print!(" - ");
            terminal.attr(term::Attr::Bold).unwrap();
            print!("{}\n", line);
            terminal.reset().unwrap();
        }
        print!("\n");
    }
    let _ = stdout.flush();
    process::exit(0);
}
