use gtk::{glib::clone, prelude::*};
pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Workshop Fetcher")
        .default_width(600)
        .default_height(400)
        .build();
    let stack = gtk::Stack::new();
    let mods_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    mods_box.set_halign(gtk::Align::Center);
    mods_box.set_valign(gtk::Align::Center);
    mods_box.set_vexpand(true);
    let buttons_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    let list_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    // https://github.com/gtk-rs/gtk4-rs/blob/main/examples/list_view_apps_launcher/main.rs
    let list_model = gtk::gio::ListStore::new::<gtk::StringObject>();
    let list_factory = gtk::SignalListItemFactory::new();
    list_factory.connect_setup(move |_list_factory, item| {
        // according to comment on the example linked above the following line is unneeded on this version of gtk, but compiler doesnt think so
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = gtk::Label::new(None);
        label.set_xalign(0.0);
        item.set_child(Some(&label));
    });
    list_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let list_string = item.item().and_downcast::<gtk::StringObject>().unwrap();
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        child.set_label(&list_string.string());
    });
    let selection_model = gtk::SingleSelection::new(Some(list_model.clone()));
    let list_view = gtk::ListView::new(Some(selection_model), Some(list_factory));
    list_box.append(&list_view);
    //list_model.append(&gtk::StringObject::new("123456"));
    //
    //
    let entry_add = gtk::Entry::builder().placeholder_text("AppID of mod to add").build();
    buttons_box.append(&entry_add);
    let button_add = gtk::Button::builder().label("Add mod").build();
    button_add.connect_clicked(clone!(
        #[weak]
        entry_add,
        #[weak]
        list_model,
        move |_btn| {
            let text = entry_add.text();
            list_model.append(&gtk::StringObject::new(&text));
            entry_add.set_text("");
        }
    ));
    buttons_box.append(&button_add);
    let entry_gameid = gtk::Entry::builder().placeholder_text("AppID of the game").build();
    buttons_box.append(&entry_gameid);
    let button_download = gtk::Button::builder().label("Download mods").build();
    button_download.connect_clicked(clone!(
        #[weak]
        list_model,
        #[weak]
        entry_gameid,
        #[weak]
        window,
        move |_btn| {
            // Note for later, extracting stuff from the list store 
            /*
            if let Some(item) = list_model.item(0) {
                println!("{}", item.downcast::<gtk::StringObject>().unwrap().string());
                let mut loop_item = 1;
                loop {
                    if let Some(item) = list_model.item(loop_item) {
                        println!("{}", item.downcast::<gtk::StringObject>().unwrap().string());
                        loop_item += 1;
                    } else {
                        break;
                    }
                }
            }
            */
            if entry_gameid.text() == "" {
                let dialog = gtk::AlertDialog::builder().message("You need to specify the game AppID").build();
                dialog.show(Some(&window.clone()));
            }
        }
    ));
    buttons_box.append(&button_download);
    //
    buttons_box.set_valign(gtk::Align::Center);
    list_box.set_valign(gtk::Align::Center);
    mods_box.append(&buttons_box);
    mods_box.append(&list_box);
    //
    let collection_box= gtk::Box::new(gtk::Orientation::Vertical, 6);
    collection_box.set_halign(gtk::Align::Center);
    collection_box.set_valign(gtk::Align::Center);
    //
    let entry_collection = gtk::Entry::builder().placeholder_text("URL of the collection").build();
    collection_box.append(&entry_collection);
    let entry_gameid = gtk::Entry::builder().placeholder_text("AppID of the game").build();
    collection_box.append(&entry_gameid);
    let button_download = gtk::Button::builder().label("Download collection").build();
    button_download.connect_clicked(clone!(
        #[weak]
        entry_collection,
        #[weak]
        entry_gameid,
        #[weak]
        window,
        move |_btn| {
            if entry_collection.text() == "" {
                let dialog = gtk::AlertDialog::builder().message("You need to specify the collection URL").build();
                dialog.show(Some(&window.clone()));
            }
            if entry_gameid.text() == "" {
                let dialog = gtk::AlertDialog::builder().message("You need to specify the game AppID").build();
                dialog.show(Some(&window.clone()));
            }
        }
    ));
    collection_box.append(&button_download);
    //
    stack.add_titled(&mods_box, Some("Download Mods"), "Download Mods");
    stack.add_titled(&collection_box, Some("Download Collection"), "Download Collection");
    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));
    let base_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    base_box.append(&stack_switcher);
    base_box.append(&stack);
    window.set_child(Some(&base_box));
    window.present();
}