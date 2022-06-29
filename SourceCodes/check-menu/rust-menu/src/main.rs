
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Window, Box, Button, ListBox, ListBoxRow, Label, CheckButton, Orientation};
//use gtk_sys::{Application, ApplicationWindow, Box, Button, ListBox, ListBoxRow, Label, CheckButton, Orientation};

/*

   We have rows. That is it.

   row [
       icons
       type
       ]

*/

/*
fn AddCheckBoxRow(label_text: &str) {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 50);
        row.add(&hbox);
        let label = Label::new(Some(label_text));
        let check = CheckButton::new();
        hbox.pack_start(&label, false, true, 0);
        hbox.pack_start(&check, false, true, 0);

        listbox.add(&row);
}
*/

fn main() {
    if gtk::init().is_err() { //Initialize Gtk before doing anything with it
        panic!("Can't init GTK");
    }

    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

            //Destroy window on exit
        //window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });


        //gtk_sys::gtk_window_present(&mut (window as Window));

        let button = gtk::Button::with_label("OK");

        button.connect_clicked(move |button| {
            //let popup_window = gtk::Window::new(gtk::WindowType::Popup);
            let popup_window = gtk::Window::builder()
                .type_(gtk::WindowType::Popup)
                .title("Popup BB")
                .window_position(gtk::WindowPosition::Mouse)
                .build();

            let box_outer = Box::new(Orientation::Vertical, 6);
            popup_window.add(&box_outer);

            let listbox = ListBox::new();
            box_outer.pack_start(&listbox, true, true, 0);

            /* Add second row */
            let row = ListBoxRow::new();
            let hbox = Box::new(Orientation::Horizontal, 50);
            row.add(&hbox);
            let label = Label::new(Some("Option 2"));
            let check = CheckButton::new();
            hbox.pack_start(&label, false, true, 0);
            hbox.pack_start(&check, false, true, 0);

            listbox.add(&row);

            popup_window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });

            /* Add Exit Button */
            let exit_button = gtk::Button::with_label("Exit");
            exit_button.connect_clicked(move |button| {std::process::exit(0)});

            listbox.add(&exit_button);

            /* Draw Popup Menu */
            popup_window.show_all();

            /*
            let menu = gtk::Menu::builder()
                .vexpand(true)
                .build();

            /* Add second row */
            let row = gtk::MenuItem::new();
            let hbox = Box::new(Orientation::Horizontal, 50);
            let label = Label::new(Some("Option 2"));
            let check = CheckButton::new();
            hbox.pack_start(&label, false, true, 0);
            hbox.pack_start(&check, false, true, 0);

            menu.add(&row);
            row.add(&hbox);

            menu.show_all();
            */

        });


        //window.add(&button);

        let box_outer = Box::new(Orientation::Vertical, 6);
        window.add(&box_outer);

        let listbox = ListBox::new();
        box_outer.pack_start(&listbox, true, true, 0);


        let row = ListBoxRow::new();
        row.add(&button);
        listbox.add(&row);


        /* Add first row */
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 50);
        row.add(&hbox);
        let label = Label::new(Some("Option 1"));
        let check = CheckButton::new();
        hbox.pack_start(&label, true, true, 0);
        hbox.pack_start(&check, false, true, 0);

        listbox.add(&row);

        /* Add second row */
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 50);
        row.add(&hbox);
        let label = Label::new(Some("Option 2"));
        let check = CheckButton::new();
        hbox.pack_start(&label, false, true, 0);
        hbox.pack_start(&check, false, true, 0);

        listbox.add(&row);

        window.show_all();
    });

    application.run();
}
