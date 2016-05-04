use gtk;
use gtk::prelude::*;
use gdk::enums::key;
use phoronix::article::Article;
use phoronix::homepage;

pub fn launch_gtk() {
    gtk::init().unwrap_or_else(|_| panic!("phoronix-reader: failed to initialize GTK."));

    // Create a ListBox that contains all of our Phoronix articles
    let list_box = gtk::ListBox::new();
    let articles = refresh_articles(&homepage::offline());
    for article in articles {
        let row = gtk::ListBoxRow::new();
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::new_with_label(&url, Some(article.title.as_str()));
        title_and_url.set_halign(gtk::Align::Start);
        row.add(&title_and_url);
        list_box.insert(&row, -1);
    }

    // Configure a container for the scrollable window that contains the ListBox.
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.add(&list_box);

    // Configure the scrollable window that contains the container.
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_min_content_width(600);
    scrolled_window.add(&container);

    // Configure the Window that will hold the scrollable window.
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(600, 800);
    window.set_title("Phoronix Reader");
    window.add(&scrolled_window);
    window.show_all();

    // Quit the program when the program has been exited
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Define custom actions on keypress
    window.connect_key_press_event(move |_, key| {
        match key.get_keyval() {
            key::Escape => gtk::main_quit(),
            _ => ()
        }
        gtk::Inhibit(false)
    });

    gtk::main();
}

#[inline]
fn refresh_articles(html: &str) -> Vec<Article> {
    Article::get_articles(html)
}
