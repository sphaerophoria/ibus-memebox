use glib::subclass::prelude::*;
use ibus::{Bus, BusExt, Component, ComponentExt, EngineDesc, FactoryExt};

mod cube_drawer;
mod engine;
use engine::MemeboxEngine;

fn main() {
    pretty_env_logger::init();
    let bus = Bus::new();
    let mainloop = glib::MainLoop::new(None, false);

    let factory = ibus::Factory::new(&bus.get_connection().unwrap());
    factory.add_engine("memebox", MemeboxEngine::get_type());

    if cfg!(debug_assertions) {
        // Debug code for if we exec outside of ibus
        let enginedesc = EngineDesc::new("memebox", "", "", "", "", "", "", "");

        let component = Component::new("", "", "", "", "", "", "", "");
        component.add_engine(&enginedesc);
        bus.register_component(&component);
    } else {
        bus.request_name("org.freedesktop.IBus.memebox", 0);
    }

    mainloop.run();
}
