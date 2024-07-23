use slint::Weak;

slint::slint! {
import { Button , VerticalBox} from "std-widgets.slint";

export component App inherits Window {
       title: "Mysh";
in property <int> counter: 0;
callback clicked <=> btn.clicked;
VerticalBox {
Text { text: "Hi Mysh " + counter; }
btn := Button { text: "yay"; }
}
}
}
fn main() {
    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();
    app.on_clicked(move || {
        let app: App = weak.upgrade().unwrap();
        app.set_counter(app.get_counter() + 1);
    });
    app.run().unwrap();
    println!("Hello Mysh");
}



