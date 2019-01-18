use std::collections::HashMap;

use crate::planet::Planet;
use crate::ship::Ship;
use crate::goods::Goods;
use crate::game::Game;

#[derive(Debug)]
pub struct Player {
    name: String,
    pub money: i64,
    pub ship: Ship,
    pub planet_index: usize,
    pub galaxy_name: String,
    pub planet_name: String,
    pub goods_count:u32,
    //飞船货仓
    cargo:HashMap<String,Goods>,
    //由于玩家输入的都是索引
    cargo_name_list:Vec<String>,
    pub year:f32,
    pub day:f32,
    kill:i32,
}

impl Player {
    pub fn new(user_name: &str, money: i64, ship: Ship, planet_index: usize,planet_name:&str) -> Player {
        Player {
            name: user_name.to_string(),
            money,
            ship,
            planet_index,
            galaxy_name: String::from("天狼星区"),
            planet_name:String::from(planet_name),
            goods_count: 0,
            year:0.0,
            day:0.0,
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
        format!("--状态--\nMoney：{}，旅行时间： {} 年 {} 天\n位置：{} -- {}，旗舰：{}\n燃料： {}，飞行状态：{}\n装甲：{}，杀敌：{}\n空余货仓：{}", self.money, self.year, self.day,self.galaxy_name,planet.name,self.ship.name,self.ship.fuel,string,self.ship.hp,self.kill,count)
    }




    pub fn sell_goods(&mut self,index:usize) -> String
    {
        //out of range check
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
        if self.money < total_price as i64 {
            return String::from("余额不足");
        }
        if surplus < goods.quantity {
            return String::from("货仓空间不足");
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
            inventory.push_str(&format!("\n物品名：{}，库存：{}",v.0,v.1.quantity));
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
