use gtk::{gio, glib::clone, prelude::*};
use adw::{self, prelude::{AdwApplicationWindowExt, AdwDialogExt, AlertDialogExt, AlertDialogExtManual}};
pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Workshop Fetcher")
        .default_width(600)
        .default_height(400)
        .build();
    let stack = adw::ViewStack::new();
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
        #[weak]
        window,
        move |_btn| {
            let text = entry_add.text();
            if text == "" {
                let dialog = adw::AlertDialog::new(Some("Mod AppID is missing"), Some("You need to specify the mod AppID"));
                dialog.add_response("cancel", "Cancel");
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                dialog.present(Some(&window));
                return;
            } 
            list_model.append(&gtk::StringObject::new(&text));
            entry_add.set_text("");
        }
    ));
    buttons_box.append(&button_add);
    //let entry_gameid = gtk::Entry::builder().placeholder_text("AppID of the game").build();
    //buttons_box.append(&entry_gameid);
    let button_download = gtk::Button::builder().label("Download mods").build();
    button_download.connect_clicked(clone!(
        #[weak]
        list_model,
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
            /*if entry_gameid.text() == "" {
                let dialog = gtk::AlertDialog::builder().message("You need to specify the game AppID").build();
                dialog.show(Some(&window.clone()));
            }*/
            let ask_dialog = adw::AlertDialog::new(Some("Please specify the game AppID"), Some(""));
            ask_dialog.add_response("continue", "Continue");
            ask_dialog.set_default_response(Some("continue"));
            let entry_gameid = gtk::Entry::builder().placeholder_text("AppID of the game").build();
            ask_dialog.set_extra_child(Some(&entry_gameid));
            ask_dialog.connect_response(
                Some("continue"), 
                clone!(
                    #[weak]
                    entry_gameid,
                    #[weak]
                    list_model,
                    move |_, _| {
                        if entry_gameid.text() == "" {
                            //TODO: handle this
                        }
                        let mut vec = Vec::new();
                        if let Some(item) = list_model.item(0) {
                            //println!("{}", item.downcast::<gtk::StringObject>().unwrap().string());
                            vec.push(item.downcast::<gtk::StringObject>().unwrap().string());
                            let mut loop_item = 1;
                            loop {
                                if let Some(item) = list_model.item(loop_item) {
                                    //println!("{}", item.downcast::<gtk::StringObject>().unwrap().string());
                                    vec.push(item.downcast::<gtk::StringObject>().unwrap().string());
                                    loop_item += 1;
                                } else {
                                    break;
                                }
                            }
                        } else {
                            //TODO: handle this
                        }
                        //test:
                        println!("{}", entry_gameid.text());
                        for n in vec.into_iter() {
                            println!("{}", n)
                        }
                    }
                ));
                ask_dialog.present(Some(&window));
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
                let dialog = adw::AlertDialog::new(Some("Collection URL is missing"), Some("You need to specify the game collection URL"));
                dialog.add_response("cancel", "Cancel");
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                dialog.present(Some(&window));
                return;
                // return so it cant do both dialogs at once
            }
            if entry_gameid.text() == "" {
                let dialog = adw::AlertDialog::new(Some("Game AppID is missing"), Some("You need to specify the game AppID"));
                dialog.add_response("cancel", "Cancel");
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                dialog.present(Some(&window));
            }
        }
    ));
    collection_box.append(&button_download);
    //
    stack.add_titled(&mods_box, Some("Download Mods"), "Download Mods");
    stack.add_titled(&collection_box, Some("Download Collection"), "Download Collection");
    let stack_switcher = adw::ViewSwitcher::new();
    stack_switcher.set_policy(adw::ViewSwitcherPolicy::Wide);
    stack_switcher.set_stack(Some(&stack));
    let base_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    base_box.append(&adw::HeaderBar::new());
    base_box.append(&stack_switcher);
    base_box.append(&stack);
    window.set_content(Some(&base_box));
    window.present();
}