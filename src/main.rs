use gdk;
use gtk::prelude::*;
use gtk::{Button, ScrolledWindow, TextView, Window, WindowType};
use pango::{FontDescription, FontMap};
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Life");
    window.set_default_size(800, 600);
    window.set_resizable(false);
    window.set_decorated(true);

    // Create a scrolled window to contain the text view
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true); // Allow vertical expansion
    scrolled_window.set_hexpand(true); // Allow horizontal expansion

    // Create a text view widget for multiline input
    let text_view = TextView::new();
    text_view.set_cursor_visible(true);
    text_view.set_wrap_mode(gtk::WrapMode::WordChar); // Enable word wrapping

    // Set text color to yellow and font to 'Roboto Mono'
    text_view.override_font(&FontDescription::from_string("Monaco 15 @‍wght=200"));

    // Set background color
    text_view.override_background_color(
        gtk::StateFlags::NORMAL,
        Some(&gdk::RGBA {
            red: 0.18,
            green: 0.18,
            blue: 0.18,
            alpha: 1.0,
        }),
    );

    // Set text color
    text_view.override_color(
        gtk::StateFlags::NORMAL,
        Some(&gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),
    );

    text_view.override_cursor(
        Some(&gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),
        Some(&gdk::RGBA {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),
    );

    // Add the text view to the scrolled window

    scrolled_window.add(&text_view);

    // Add the scrolled window to the main window
    window.add(&scrolled_window);

    let close_button = Button::with_label("×");
    close_button.set_size_request(30, 30);

    // Connect the "delete-event" signal to quit the application
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Connect the close button to close the window
    close_button.connect_clicked(move |_| {
        gtk::main_quit();
    });

    // Create a box to hold the close button
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&close_button);

    // let text_view_clone = text_view.clone();
    // window.connect_key_press_event(move |_, key| {
    //     // Check for `Cmd + Enter` (on Mac) or `Ctrl + Enter` (on others)
    //     let cmd_or_ctrl = key.state().contains(ModifierType::META_MASK)
    //         || key.state().contains(ModifierType::CONTROL_MASK);
    //     if cmd_or_ctrl && key.keyval() == gdk::keys::constants::Return {
    //         // Get the buffer from the TextView
    //         if let Some(buffer) = text_view_clone.buffer() {
    //             // Extract the text from the buffer
    //             let (start, end) = buffer.bounds();
    //             if let Ok(text) = buffer.text(&start, &end, false) {
    //                 println!("Extracted text: {}", text); // Store it or use it as needed
    //             }
    //         }
    //         Inhibit(true) // Prevent default behavior
    //     } else {
    //         Inhibit(false)
    //     }
    // });

    // Show all widgets in the window
    window.show_all();

    // Run the GTK main loop
    gtk::main();
}
