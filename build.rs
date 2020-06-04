extern crate rust_qt_binding_generator;

use rust_qt_binding_generator::build::QtModule;

fn main() {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    rust_qt_binding_generator::build::Build::new(&out_dir)
        .bindings("bindings.json")
        .qrc("qml.qrc")
        .cpp("src/main.cpp")
        .include_path("/usr/include/glib-2.0")
        .include_path("/usr/lib/glib-2.0/include")
        .module(QtModule::Gui)
        .module(QtModule::Qml)
        .module(QtModule::Quick)
        .compile("uilau");
}
