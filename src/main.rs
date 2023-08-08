use adw::{Application, ApplicationWindow, ViewStack};
use error_chain::error_chain;
use gtk::gdk::Display;
use gtk::{gio, glib, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::{prelude::*, Button};
use std::io::Read;
use std::rc::Rc;

const APP_ID: &str = "com.buttg.RiotLauncher";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_app| load_css());

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // The CSS "magic" happens here.
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../assets/styles.css"));
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let ui_src = include_str!("../assets/launcher.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("main_window").expect("Couldn't get window");
    app.add_window(&window);

    // Present window
    window.present();

    // Setup bindings
    let stack: ViewStack = builder.object("stk1").expect("Couldn't get stack");
    let login_btn: Button = builder
        .object("page-login-btn")
        .expect("Couldn't get login button");

    setup_login(&login_btn, stack.clone().into());
}

fn setup_login(btn: &Button, stack: Rc<ViewStack>) {
    btn.connect_clicked(move |btn: &Button| {
        login(btn).expect("Something went wrong");
        stack.set_visible_child_name("main");
    });
}

fn login(btn: &Button) -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
