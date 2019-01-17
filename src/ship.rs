#[derive(Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub price: i32,
    pub describe: String,
    pub hp: i32,
    pub max_hp: i32,
    pub cargo: i32,
    pub speed: i32,
    pub fuel: i32,
    pub max_fuel: i32,
}

impl Ship {
    pub fn new(name: &str, hp: i32, cargo: i32, speed: i32, price: i32, fuel: i32) -> Ship {
        Ship {
            name: name.to_string(),
            hp,
            max_hp: hp,
            cargo,
            speed,
            price,
            fuel,
            max_fuel: fuel,
            describe: String::from(""),
        }
    }

    pub fn calculate_fuel(&mut self, fuel_value: i32) -> bool {
        let tmp_fuel = self.fuel + fuel_value;
        if tmp_fuel < 0 {
            return false;
        }
        self.fuel = tmp_fuel;
        true
    }

    pub fn calculate_hp(&mut self, hp_value: i32) -> bool {
        let tmp_hp = self.hp + hp_value;
        if tmp_hp < 0 {
            return false;
        }
        self.hp = tmp_hp;
        true
    }

    // pub fn refuel(&mut self, &player:Player) -> String
    // {
    //     let refuel_price = self.max_fuel - self.fuel;
    //     let mut msg = "";
    //     if player.cash - refuel_price < 0
    //     {
    //         msg = "没钱修理";
    //     }
    //     else
    //     {
    //         let tmp_msg = format!("燃料已加满，花费{}",refuel_price);
    //         msg = &tmp_msg;
    //         player.add_credits(-refuel_price);
    //         self.fuel = self.max_fuel;
    //     }
    //     msg.to_string()
    // }

    // pub fn repair(&mut self, &player:Player) -> String
    // {
    //     let repair_price = self.max_hp - self.hp;
    //     let mut msg = "";
    //     if player.cash - repair_price < 0
    //     {
    //         msg = "没钱修理";
    //     }
    //     else
    //     {
    //         let tmp_msg = format!("修理完成，花费{}",repair_price);
    //         msg = &tmp_msg;
    //         player.add_credits(-repair_price);
    //         self.hp = self.max_hp;
    //     }
    //     msg.to_string()
    // }
}
