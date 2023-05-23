use gtk::prelude::{
    DialogExt, GtkWindowExt, WidgetExt,
};
use relm4::*;

pub struct DialogModel {
    hidden: bool,
}

#[derive(Debug)]
pub enum DialogInput {
    Show,
    Accept,
    Cancel,
}

#[derive(Debug)]
pub enum DialogOutput {
    Close,
}

#[relm4::component(pub)]
impl SimpleComponent for DialogModel {
    type Init = bool;
    type Input = DialogInput;
    type Output = DialogOutput;

    view! {
        gtk::MessageDialog {
            set_modal: true,
            #[watch]
            set_visible: !model.hidden,
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: ("Close", gtk::ResponseType::Accept),
            add_button: ("Cancel", gtk::ResponseType::Cancel),
            connect_response[sender] => move |_, resp| {
                sender.input(if resp == gtk::ResponseType::Accept {
                    DialogInput::Accept
                } else {
                    DialogInput::Cancel
                })
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DialogModel { hidden: params };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DialogInput::Show => self.hidden = false,
            DialogInput::Accept => {
                self.hidden = true;
                sender.output(DialogOutput::Close).unwrap()
            }
            DialogInput::Cancel => self.hidden = true,
        }
    }
}