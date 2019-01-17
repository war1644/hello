use std::collections::HashMap;

use crate::planet::Planet;
use crate::ship::Ship;
use crate::goods::Goods;

#[derive(Debug)]
pub struct Player {
    name: String,
    pub money: i32,
    pub ship: Ship,
    pub planet_index: usize,
    galaxy: String,
    pub goods_count:i32,
    goods_list:HashMap<String,Goods>,
    year:i32,
    day:i32,
    kill:i32,
}

impl Player {
    pub fn new(user_name: &str, money: i32, ship: Ship, planet_index: usize) -> Player {
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
            goods_list:HashMap::new(),
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

    pub fn sell_goods(index:usize) -> String
    {
        string tmpName = goodNameList[index];
        if(!cargo.ContainsKey(tmpName))
        {
        return String::from("没有这个货物");
        }
        Good good = cargo[tmpName];
        int totalPrice = good.price * good.quantity;
        goodsCount -= good.quantity;
        RemoveCargoGood(tmpName);
        AddCredits(totalPrice);
        String::from("{name} 已卖")
    }

    pub fn buy_goods(&mut self,goods:Goods, quantity:usize) -> String
    {
        let total_price = goods.price * quantity as u32;
        let surplus = self.ship.cargo - self.goods_count;
        if credits >= total_price && goods.quantity <= surplus
        {
            goodsCount += quantity;
            good.quantity = quantity;
            self.add_cargo_goods(goods);
            self.add_credits(-total_price);
            return String::from("购买成功")
        }
        if credits < total_price {
            return String::from("钱不够");
        }
        String::from("货仓空间不够")
    }

    pub fn add_goods(&mut self, goods:Goods, quantity:u32) -> String
    {
        let surplus = self.ship.cargo - self.goods_count;
        if good.quantity <= surplus
        {
            self.goods_count = self.goods_count + quantity as i32;
            goods.quantity = quantity;
            self.add_cargo_goods(goods);
            return String::from("已存入货仓");
        }
        String::from("货仓空间不够")
    }

    public List<string> CargoInventory ()
    {
    var inventory = new List<string>();

    foreach (var item in cargo)
    {
    inventory.Add($"物品名：{item.Value.name}，库存：{item.Value.quantity}");
    }
    return inventory;
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
