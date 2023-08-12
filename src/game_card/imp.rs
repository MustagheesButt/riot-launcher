use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Picture, Image, Label};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/res/game-card.ui")]
pub struct GameCard {
    #[template_child]
    pub image: TemplateChild<Picture>,
    #[template_child]
    pub icon: TemplateChild<Image>,
    #[template_child]
    pub name: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GameCard {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GameCard";
    type Type = super::GameCard;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for GameCard {}

// Trait shared by all widgets
impl WidgetImpl for GameCard {}

// Trait shared by all boxes
impl BoxImpl for GameCard {}
