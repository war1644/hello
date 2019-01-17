use crate::goods::Goods;
use crate::ship::Ship;

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    pub galaxy: String,
    pub fleet: Vec<Ship>,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
    pub goods: Vec<Goods>,
}

impl Planet {
    pub fn new(name: &str, x: i32, y: i32, galaxy_name: &str) -> Planet {
        Planet {
            name: name.to_string(),
            galaxy: galaxy_name.to_string(),
            x,
            y,
            fleet: Vec::new(),
            goods: Vec::new(),
            distance: 0,
        }
    }

    pub fn add_fleet(&mut self, fleet: &mut Vec<Ship>) {
        self.fleet.append(fleet);
    }
}
