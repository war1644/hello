use crate::planet::Planet;
use std::collections::HashMap;

pub struct Galaxy {
    pub name_list: [String; 6],
    pub list: HashMap<String, Vec<Planet>>,
    pub current: String,
}

impl Galaxy {
    pub fn new() -> Galaxy {
        Galaxy {
            name_list: [
                String::from("人马座"),
                String::from("烈阳星区"),
                String::from("天狼星区"),
                String::from("北落师门"),
                String::from("PLA"),
                String::from("北极星区"),
            ],
            list: HashMap::new(),
            current: String::from("天狼星区"),
        }
    }

    pub fn get_planet(&self, index:usize ) -> &Planet {
        let planets = self.list.get(&self.current);
        if let Some(v) = planets {
            return &v[index];
        }
        panic!("index error {}",index);
    }

    pub fn get_planet_list(&self, index_key: &str) -> String {
        let planets = self.list.get(index_key);
        let mut inventory = String::new();
        if let Some(v) = planets {
            for (i,planet) in v.into_iter().enumerate() {
                inventory.push_str(&format!(
                    "\n{} 星球名：{}，位置：[{}, {}]， 距离：{}",i,
                    planet.name, planet.x, planet.x,planet.distance
                ));
            }
        }
        inventory
    }


    pub fn map(&self) -> String {
        let mut inventory = String::new();
        for (key, value) in &self.list {
            let mut tmp_str = format!("\n* {} |", key);
            for planet in value.iter() {
                tmp_str.push_str(&format!(
                    "\n    --{}，防御舰船：{}",
                    planet.name,
                    planet.fleet.len()
                ));
            }
            inventory.push_str(&tmp_str);
        }
        inventory
    }

    //galaxy jump
    pub fn jump (&mut self){
        match &self.current[..] {
            "人马座" => {
                self.current = "烈阳星区".to_string();
            }
            "烈阳星区" => {
                self.current = "天狼星区".to_string();
            }
            "天狼星区" => {
                self.current = "北落师门".to_string();
            }
            "北落师门" => {
                self.current = "PLA".to_string();
            }
            "PLA" => {
                self.current = "北极星区".to_string();
            }
            "北极星区" => {
                self.current = "人马座".to_string();
            }
            _ => {
                panic!("jump error");
            }
        }
    }

    // pub fn showPlants() ->
    // {
    //     if(Galaxy.plantsView.length) return Galaxy.plantsView;
    //     Galaxy.plantsView = [];
    //     Galaxy.currentPlants.forEach((v) => {
    //         //获取渲染
    //         Galaxy.plantsView.push(v.getView());
    //     });
    //     return Galaxy.plantsView;
    // }
}
