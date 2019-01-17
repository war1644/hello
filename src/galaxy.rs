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

    pub fn get_planet_list(&self, index_key: &str) -> Vec<String> {
        let planets = self.list.get(index_key);
        let mut inventory = Vec::new();
        if let Some(v) = planets {
            for planet in v {
                inventory.push(format!(
                    "星球名：{}，位置：[{}, {}]",
                    planet.name, planet.x, planet.x
                ));
            }
        }
        inventory
    }



    pub fn map(&self) -> Vec<String> {
        let mut inventory = Vec::new();
        for (key, value) in &self.list {
            let mut tmp_str = format!("* {} |", key);
            for planet in value.iter() {
                tmp_str.push_str(&format!(
                    "\n    --{}，防御舰船：{}",
                    planet.name,
                    planet.fleet.len()
                ));
            }
            inventory.push(tmp_str);
        }
        inventory
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
