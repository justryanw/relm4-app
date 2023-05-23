use gtk::prelude::{
    ApplicationExt, GtkWindowExt,
};
use relm4::*;

mod header;
use header::{ HeaderModel, HeaderOutput };

mod dialog;
use dialog::{ DialogModel, DialogInput, DialogOutput };

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

struct AppModel {
    mode: AppMode,
    header: Controller<HeaderModel>,
    dialog: Controller<DialogModel>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<HeaderModel> =
            HeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                    HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                    HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
                });

        let dialog = DialogModel::builder()
            .transient_for(root)
            .launch(true)
            .forward(sender.input_sender(), |msg| match msg {
                DialogOutput::Close => AppMsg::Close,
            });

        let model = AppModel {
            mode: params,
            header,
            dialog,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                self.dialog.sender().send(DialogInput::Show).unwrap();
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }
    }
}

fn main() {
    let relm = RelmApp::new("ewlm4.test.components");
    relm.run::<AppModel>(AppMode::View);
}
