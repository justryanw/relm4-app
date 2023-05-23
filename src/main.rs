use relm4::*;

mod app;
mod header;
mod dialog;

use crate::app::{AppModel, AppMode};

fn main() {
    let relm = RelmApp::new("ewlm4.test.components");
    relm.run::<AppModel>(AppMode::View);
}
