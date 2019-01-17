// #![allow(unused_variables)]
#![allow(unused)]
mod galaxy;
mod game;
mod goods;
mod planet;
mod player;
mod ship;

use crate::game::Game;
// use crate::goods::Goods;
extern crate iui;
use iui::controls::{Button, Group, Label, LayoutGrid, VerticalBox};
use iui::prelude::*;

fn main() {
    // Game::new().start();
    let ui = UI::init().expect("UI init failed");
    let mut win = Window::new(&ui, "Fearless Void", 400, 400, WindowType::NoMenubar);
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);
    let mut button = Button::new(&ui, "Start");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
        }
    });
    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });
    let mut label_text = String::new();
    label_text.push_str("无畏虚空\n");
    label_text.push_str("By: 路漫漫 ");
    let label = Label::new(&ui, &label_text);

    vbox.append(&ui, label, LayoutStrategy::Compact);
    // group.set_child(&ui, group_vbox);

    vbox.append(&ui, button, LayoutStrategy::Compact);
    vbox.append(&ui, quit_button, LayoutStrategy::Compact);
    // vbox.append(&ui, group, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}
