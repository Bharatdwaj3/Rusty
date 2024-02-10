use gtk::prelude::*;
use gtk::{Window, WindowType, Notebook, Label};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Create a window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Tabs Example");
    window.set_default_size(400, 300);

    // Create a notebook (container for tabs)
    let notebook = Notebook::new();
    
    // Create tabs
    let tab1_label = Label::new(Some("Tab 1"));
    let tab2_label = Label::new(Some("Tab 2"));
    let tab3_label = Label::new(Some("Tab 3"));

    // Add tabs to the notebook
    notebook.append_page(&tab1_label, None);
    notebook.append_page(&tab2_label, None);
    notebook.append_page(&tab3_label, None);

    // Add the notebook to the window
    window.add(&notebook);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
