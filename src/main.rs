use std::ffi::CStr;
use qmetaobject::*;

#[allow(non_snake_case)]
#[derive(Default, QObject)]
struct Greeter {
    base : qt_base_class!(trait QObject),
    name : qt_property!(QString; NOTIFY name_changed),
    name_changed : qt_signal!(),
    compute_greetings : qt_method!(fn compute_greetings(&self, verb : String) -> QString {
        return (verb + " " + &self.name.to_string()).into()
    })
}

fn main() {
    qml_register_type::<Greeter>(
        CStr::from_bytes_with_nul(b"Greeter\0").unwrap(),
        1,
        0,
        CStr::from_bytes_with_nul(b"Greeter\0").unwrap()
    );
    let mut engine = QmlEngine::new();
    engine.load_data(r#"
        import QtQuick 2.6; import QtQuick.Window 2.0;
        import Greeter 1.0
        Window {
            visible: true;
            title: "Hello, QMetaObject";
            Greeter { id: greeter; name: 'World'; }
            Text { anchors.centerIn: parent; text: greeter.compute_greetings('hello'); }            
        }
    "#.into());
    engine.exec();
}
