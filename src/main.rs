use relm4::{
    gtk::{
        self,
        glib::clone,
        traits::{BoxExt, ButtonExt, GtkWindowExt},
        Orientation,
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, RelmApp,
};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = u8;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple App")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
        counter: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        let vbox = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        inc_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Increment);
        }));

        dec_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Decrement);
        }));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => {
                self.counter = self.counter.saturating_add(1);
            }
            AppInput::Decrement => {
                self.counter = self.counter.saturating_sub(1);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_maual");
    app.run::<AppModel>(0);
}
