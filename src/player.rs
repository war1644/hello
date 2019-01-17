use std::collections::HashMap;

use crate::planet::Planet;
use crate::ship::Ship;
use crate::goods::Goods;

#[derive(Debug)]
pub struct Player {
    name: String,
    pub money: i64,
    pub ship: Ship,
    pub planet_index: usize,
    galaxy: String,
    pub goods_count:u32,
    //飞船货仓
    cargo:HashMap<String,Goods>,
    //由于玩家输入的都是索引
    cargo_name_list:Vec<String>,
    year:i32,
    day:i32,
    kill:i32,
}

impl Player {
    pub fn new(user_name: &str, money: i64, ship: Ship, planet_index: usize) -> Player {
        Player {
            name: user_name.to_string(),
            money,
            ship,
            planet_index,
            galaxy: String::from("天狼星区"),
            goods_count: 0,
            year:0,
            day:0,
            kill:0,
            cargo:HashMap::new(),
            cargo_name_list:vec![]
        }
    }

    pub fn get_state(&self,(docked,planet):(bool,&Planet)) -> String
    {
        let count = self.ship.cargo-self.goods_count;
        let mut string = "飞行中";
        if docked {
            string = "停靠中";
        }
        format!("--状态--\nMoney：{}，旅行时间： {} 年 {} 天\n位置：{} -- {}，旗舰：{}\n燃料： {}，飞行状态：{}\n装甲：{}，杀敌：{}\n空余货仓：{}", self.money, self.year, self.day,self.galaxy,planet.name,self.ship.name,self.ship.fuel,string,self.ship.hp,self.kill,count)
//        state
    }

    pub fn set_goto_plant(&mut self, to_planet_index: usize) {
        self.set_plant(to_planet_index);

        // int distance = CalculatePlanetsDistance(toPlanet);
        // if(ship.CalculateFuel(distance))
        // {
        // day += (distance/ship.speed) <= 0 ? 1 : (distance/ship.speed);
        // CalculateYears();
        // return true;
        // }
        // return false;
    }

    pub fn set_plant(&mut self, to_planet_index: usize) {
        self.planet_index = to_planet_index;
        // if(planet.name == "星系跳跃门")
        // {
        //     Game.docked = false;
        //     Game.isJump = true;
        // }
        // else
        // {
        //     Game.docked = false;
        //     Game.isJump = false;
        // }
    }

    pub fn sell_goods(&mut self,index:usize) -> String
    {
        //检查越界
        if index >= self.cargo_name_list.len() {
            return String::from("没有这个货物");
        }
        let key_name = self.cargo_name_list[index].clone();
        let goods = self.cargo.get(&key_name);
        if let Some(v) = goods {
            let total_price = v.price * v.quantity;
            self.goods_count -= v.quantity;
            self.add_money(total_price as i64);
            self.cargo.remove(&key_name);
            self.cargo_name_list.remove(index);
        }
        format!("{} 已卖",key_name)
    }

    pub fn buy_goods(&mut self,goods:Goods) -> String
    {
        let total_price = goods.price * goods.quantity;
        //飞船仓库剩余空间
        let surplus = self.ship.cargo - self.goods_count;
        if self.money >= total_price as i64 {
            return  String::from("余额不足")
        }
        if surplus >= goods.quantity {
            return  String::from("货仓空间不足")
        }
        self.goods_count += goods.quantity;
        if !self.cargo.contains_key(&goods.name) {
            self.cargo_name_list.push(goods.name.clone());
        }
        self.add_cargo_goods(goods);
        self.add_money(-(total_price as i64));
        String::from("购买成功")
    }

    //外部调用 非购买
    pub fn add_goods(&mut self, mut goods:Goods) -> String
    {
        let surplus = self.ship.cargo - self.goods_count;
        if surplus >= goods.quantity
        {
            self.goods_count += goods.quantity;
            if !self.cargo.contains_key(&goods.name) {
                self.cargo_name_list.push(goods.name.clone());
            }
            self.add_cargo_goods(goods);
            return String::from("已存入货仓");
        }
        String::from("货仓空间不够")
    }

    pub fn add_money(&mut self,money:i64)
    {
        self.money += money;
    }


    fn add_cargo_goods(&mut self,goods:Goods)
    {
        self.cargo.entry(goods.name.clone())
            .and_modify(|e| { e.quantity += goods.quantity })
            .or_insert(goods);
    }

    fn remove_cargo_goods(&mut self, key_name:&str)
    {
        self.cargo.remove(key_name);
    }

    pub fn cargo_inventory(&self) -> String
    {
        let mut inventory = String::new();
        for v in self.cargo.iter() {
            inventory.push_str(&format!("物品名：{}，库存：{}\n",v.1.name,v.1.quantity));
        }
        inventory
    }
}

// #[cfg(test)]
// mod tests {

//     use player::*;

//     #[test]
//     fn player_new() {
//         Player::new();
//     }
// }
