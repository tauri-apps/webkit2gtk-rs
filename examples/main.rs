extern crate gtk;
extern crate webkit2;

use gtk::{ContainerExt, Inhibit, WidgetExt, Window, WindowType};
use webkit2::WebView;

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let webview = WebView::new();
    webview.load_uri("https://crates.io/");
    window.add(&webview);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
