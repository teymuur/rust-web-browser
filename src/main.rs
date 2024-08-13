use gtk::{Button, Box};

fn main() {
    let app = Application::new(
        Some("com.example.rust_browser"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Rust Browser");
        window.set_default_size(800, 600);

        let vbox = Box::new(Orientation::Vertical, 0);

        let url_entry = Entry::new();
        url_entry.set_placeholder_text("Enter URL");

        let go_button = Button::with_label("Go");
        let back_button = Button::with_label("Back");
        let forward_button = Button::with_label("Forward");

        let webview = WebView::new().expect("Failed to create WebView");

        vbox.pack_start(&url_entry, false, false, 0);
        vbox.pack_start(&back_button, false, false, 0);
        vbox.pack_start(&forward_button, false, false, 0);
        vbox.pack_start(&go_button, false, false, 0);
        vbox.pack_start(&webview, true, true, 0);

        window.add(&vbox);

        go_button.connect_clicked(move |_| {
            let url = url_entry.get_text().unwrap();
            webview.navigate(&url);
        });

        back_button.connect_clicked(move |_| {
            webview.go_back();
        });

        forward_button.connect_clicked(move |_| {
            webview.go_forward();
        });

        window.show_all();
    });

    app.run();
}
