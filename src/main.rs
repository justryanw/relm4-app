use relm4::{
    gtk::{
        self,
        traits::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt},
        Orientation,
    },
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    }
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => {
                self.counter = self.counter.saturating_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.saturating_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_maual");
    app.run::<AppModel>(0);
}
