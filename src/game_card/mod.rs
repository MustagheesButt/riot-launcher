mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};
// use gtk::prelude::*;
// use gtk::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
  pub struct GameCard(ObjectSubclass<imp::GameCard>)
      @extends gtk::Box, gtk::Widget,
      @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl GameCard {
    pub fn new(name: &str, image: &str, icon: &str) -> Self {
        let gc: Self = Object::builder().build();
        gc.imp().name.set_label(name);
        gc.imp().icon.set_resource(Some(icon));
        gc.imp().image.set_resource(Some(image));
        gc
    }
}
