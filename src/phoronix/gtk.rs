use gtk;
use gtk::prelude::*;
use gdk::enums::key;
use linesplit::CharSplit;
use phoronix::article::Article;
use phoronix::homepage;

/// Launches the GTK3 GUI interface
pub fn launch_gtk() {
    gtk::init().unwrap_or_else(|_| panic!("phoronix-reader: failed to initialize GTK."));

    // Create a ListBox that contains all of our Phoronix articles
    let list_box = gtk::ListBox::new();
    let articles = Article::get_articles(&homepage::offline());
    for article in articles {
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::new_with_label(&url, Some(article.title.as_str()));
        title_and_url.set_halign(gtk::Align::Start);

        let row1 = gtk::ListBoxRow::new();
        let row2 = gtk::ListBoxRow::new();
        let row3 = gtk::ListBoxRow::new();

        let details = gtk::TextView::new();
        let mut details_text = article.details.split_by_chars(80).into_iter()
            .map(|x| x + "\n").collect::<String>();
        let _ = details_text.pop();
        details.get_buffer().unwrap().set_text(&details_text);
        details.set_editable(false);
        details.set_left_margin(5);
        details.set_right_margin(5);
        details.set_halign(gtk::Align::Start);

        let summary = gtk::TextView::new();
        let mut summary_text = article.summary.split_by_chars(80).into_iter()
            .map(|x| x + "\n").collect::<String>();
        let _ = summary_text.pop();
        summary.get_buffer().unwrap().set_text(&summary_text);
        summary.set_editable(false);
        summary.set_left_margin(5);
        summary.set_right_margin(5);
        summary.set_halign(gtk::Align::Start);

        row1.add(&title_and_url);
        row2.add(&details);
        row3.add(&summary);
        row2.set_selectable(false);
        row3.set_selectable(false);
        list_box.insert(&row1, -1);
        list_box.insert(&row2, -1);
        list_box.insert(&row3, -1);
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
