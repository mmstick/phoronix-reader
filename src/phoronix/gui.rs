use phoronix::article::Article;
use phoronix::homepage;
use gtk;
use gtk::traits::*;
use gdk::enums::key;
use gdk::ffi::GdkRGBA;
use pango;

macro_rules! color {
    (white) => (GdkRGBA{red: 1f64, green: 1f64, blue: 1f64, alpha: 1f64});
    (black) => (GdkRGBA{red: 0f64, green: 0f64, blue: 0f64, alpha: 1f64});
    (green) => (GdkRGBA{red: 0.2f64, blue: 0.2f64, green: 0.5f64, alpha: 1f64});
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // Create widgets for the articles
    let article_box = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    let articles = Article::get_articles(&homepage::online());
    generate_article_widgets(&article_box, &articles);
    article_box.override_background_color(gtk::StateFlags::empty(), &color!(green));

    // Insert the articles into a scrolled window
    let scrolled_window = gtk::ScrolledWindow::new(None, None).unwrap();
    scrolled_window.set_min_content_width(600);
    scrolled_window.add(&article_box);

    // Add the scrolled window to a main window
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    configure_window(&window);
    window.add(&scrolled_window);
    window.show_all();

    // Define actions on key press
    window.connect_key_press_event(move |_, key| {
        match key.keyval as i32 {
            key::Escape => gtk::main_quit(),
            _ => ()
        }
        gtk::signal::Inhibit(false)
    });

    gtk::main();
}

// configre_window configures the given window.
fn configure_window(window: &gtk::Window) {
    window.set_title("Phoronix Reader");
    let (width, height) = (600, 500);
    window.set_default_size(width, height);
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::signal::Inhibit(true)
    });
}

// generate_article_widgets takes a vector of articles as well as a gtk::Box and fills up the gtk::Box
// with widgets generated from each article
fn generate_article_widgets(article_box: &gtk::Box, articles: &Vec<Article>) {
    let mut bold = pango::FontDescription::new();
    bold.set_weight(pango::Weight::Heavy);
    for article in articles {
        // Creates the title as a gtk::LinkButton for each article
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::new_with_label(&url, &article.title).unwrap();
        title_and_url.override_background_color(gtk::StateFlags::empty(), &color!(green));
        title_and_url.override_color(gtk::StateFlags::empty(), &color!(white));
        title_and_url.override_font(&bold);
        title_and_url.set_halign(gtk::Align::Start);
        title_and_url.set_margin_start(0);

        // Details of the article inside of a gtk::TextView
        let details = gtk::TextView::new().unwrap();
        details.set_halign(gtk::Align::Start);
        details.override_background_color(gtk::StateFlags::empty(), &color!(green));
        details.override_color(gtk::StateFlags::empty(), &color!(white));
        details.override_font(&bold);
        details.set_left_margin(10);
        details.set_right_margin(10);
        details.set_editable(false);
        details.get_buffer().unwrap().set_text(&article.details);

        // Summary of the article inside of a gtk::TextView
        let summary = gtk::TextView::new().unwrap();
        summary.set_wrap_mode(gtk::WrapMode::Word);
        summary.set_left_margin(10);
        summary.set_right_margin(10);
        summary.set_pixels_above_lines(10);
        summary.set_pixels_below_lines(10);
        summary.set_editable(false);
        summary.override_background_color(gtk::StateFlags::empty(), &color!(white));
        summary.override_color(gtk::StateFlags::empty(), &color!(black));
        summary.get_buffer().unwrap().set_text(&article.summary);

        // Attach the title+url, details and summary to the article_box
        article_box.add(&title_and_url);
        article_box.add(&details);
        article_box.add(&summary);
    }
}
