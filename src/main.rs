#![allow(unused)]
mod galaxy;
mod game;
mod goods;
mod planet;
mod player;
mod ship;
mod menu;
//mod display;

use crate::game::Game;


fn main() {

     Game::new(600,400).start();
}

