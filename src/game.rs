use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;
use std::result::Result;
use std::str::FromStr;

use crate::galaxy::Galaxy;
use crate::goods::Goods;
use crate::planet::Planet;
use crate::player::Player;
use crate::ship::Ship;
use crate::menu::Menu;
//use crate::display::Display;

static WAIT_TIME: u32 = 100;

pub struct Game {
    w: i32,
    h: i32,
    goods: [Goods; 9],
    ships: [Ship; 7],
    galaxy: Galaxy,
//    player: Option<Player>,
    docked: bool,
    user_input: String
}

impl Game {
    pub fn new(width:i32,height:i32) -> Game {
        Game {
            w: width,
            h: height,
            goods: Self::create_goods(),
            ships: Self::create_ship(),
            galaxy: Galaxy::new(),
//            player: None,
            docked:true,
            user_input:String::from("")
        }
    }

    pub fn start(&mut self) {
        self.create_galaxy();
        let ship = self.ships[1].clone();
        let player = Player::new("路漫漫", 9999, ship, 0);
        player.buy_goods(self.goods[1],5);
        self.process_logic(player);
    }

    fn get_input() -> String
    {
        let mut user_input = String::new();
        loop {
            io::stdin().read_line(&mut user_input).expect("failed");
            return user_input.trim().to_string();
        }
    }

    fn get_choice() -> usize
    {
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("failed");

            match user_input.trim().parse::<usize>() {
                Ok(number) => {return number},
                Err(_) => {
                    println!("无效输入");
                    thread::sleep(Duration::from_millis(100));
                    continue
                }
            }
        }
    }

    fn process_logic(&self,player:Player) {
        let mut choice = 0;
        loop {
            if self.docked {
                println!("\n> 已停靠，等待指令中...(帮助请输入'help') ");
            } else {
                println!("\n> 等待指令中... (帮助请输入'help')");
            }
            let user_input = &Self::get_input()[..];
            match user_input {
                "help" => {
                    println!("{}",Menu::help());
                }
                "state" => {
                    let mut current_planet= self.galaxy.get_planet(player.planet_index);
                    let state = player.get_state((self.docked,current_planet));
                    println!("{}",&state[..]);
                }

                "buy" if self.docked => {
                    let mut current_planet= self.galaxy.get_planet(player.planet_index);
                    let planet_goods = &current_planet.goods;
                    println!("{:#?}",planet_goods);
                    choice = Self::get_choice();
                    println!("{}",choice);

                    let goods = &planet_goods[choice];
                    println!("> 买多少的{}？", good.name);
                    choice = Self::get_choice();

                    let info = player.buy_goods(goods, number);
                    println!("{}",info);
                }

                _ => {
                    thread::sleep(Duration::from_millis(100));
                    continue
                }
            }
/*
            if(userInput == "buy" && docked)
            {

            }
            else if(userInput == "sell" && docked)
            {
                Display::showIndex(player.goodNameList);
                choice = GetChoice();
                string info = player.SellGood(choice);
                Display::show(info);
            }
            else if(userInput == "cargo")
            {
                Display::show(player.CargoInventory());
            }
            else if(userInput == "ship" && docked)
            {
                Display::show(ships);
                choice = GetChoice();
                string info = player.BuyShip(ships[choice]);
                Display::show(info);
            }
            else if(userInput == "help")
            {
                if(docked){
                    Display::show(Menu.docked);
                }else{
                    Display::show(Menu.help);
                }
            }
            else if(userInput == "state")
            {
                Display::show(player.GetState());
            }
            else if(userInput == "refuel" && docked)
            {
                player.ship.Refuel(player);
            }
            else if(userInput == "repair" && docked)
            {
                player.ship.Repair(player);
            }
            else if(userInput == "upgrade" && docked)
            {

            }
            else if(userInput == "simulator" && docked)
            {
                new Battle(player);
            }
            else if(userInput == "dock" && !docked)
            {
                if(player.planet.name != "星系跳跃门")
                {
                    Display::auto_show("已发出停靠请求");
                    Display::auto_show("获得停靠许可，开始停靠...");
                    docked = true;
                    Display::auto_show("停靠完成");
                }
            }
            else if(userInput == "goto" && !docked)
            {
                string galaxyName = player.planet.galaxy;
                Planet[] planets = Galaxy.list[galaxyName];
                Display::show("请选择目标星球：");
                Display::show(planets);
                choice = GetChoice();
                Display::auto_show("锁定目标星球，开始巡航...");
                int battle = rand.Next(0,9);
                if(battle>5){
                    new Battle(player);
                }
                bool success = player.SetGoToPlant(planets[choice]);
                if(success)
                {
                    Display::auto_show($"到达 {planets[choice].name}");
                }
                else
                {
                    Display::auto_show($"距离过远，需要更多的燃料");
                }
            }
            else if(userInput == "undock" && docked)
            {
                Display::auto_show("飞船已升空");
                docked = false;
            }
            else if(userInput == "save")
            {
                Save(this);
            }
            else if(userInput == "load")
            {
                Load();
            }
            else if(userInput == "jump" && !docked)
            {
                if(isJump)
                {
                    Display::auto_show("连接星际大门中...");
                    Display::auto_show("连接成功，请选择跳跃星系");
                    Display::showIndex(Galaxy::showNameList);
                    choice = GetChoice();
                    string galaxyName = Galaxy.nameList[choice];
                    if(galaxyName == player.planet.galaxy)
                    {
                        Display::auto_show("ERROR,不能选择当前星区");
                    }
                    else
                    {
                        Display::auto_show("开始跃迁...");
                        Planet planet = Galaxy.list[galaxyName][0];
                        player.SetPlant(planet);
                        Display::auto_show($"跃迁完成，欢迎来到 {galaxyName} ");
                    }
                }
                else
                {
                    Display::show("ERROR,附近未发现星际大门");
                }
            }
            else if(userInput == "exit")
            {
                Display::auto_show("关闭中...");
                isWhile = false;
            }
            else if(userInput == "map")
            {
                Display::show(self.galaxy.map());
            }*/

        }
    }



    fn create_goods() -> [Goods; 9] {
        [
            Goods::new("营养液", 200),
            Goods::new("金", 12000),
            Goods::new("钻石", 1400),
            Goods::new("矿石", 30),
            Goods::new("武器", 60),
            Goods::new("木材", 10),
            Goods::new("铜", 50),
            Goods::new("暗物质", 450000),
            Goods::new("生活包", 500),
        ]
    }

    fn create_ship() -> [Ship; 7] {
        [
            Ship::new("采矿船", 1000, 150, 3, 100000, 25),
            Ship::new("轻型货机", 500, 50, 3, 25000, 20),
            Ship::new("PLA战斗机", 300, 300, 4, 75000, 20),
            Ship::new("CR90护卫舰", 3000, 750, 7, 125000, 30),
            Ship::new("PLA驱逐舰", 50000, 500, 9, 400000, 40),
            Ship::new("帝国驱逐舰", 50000, 1000, 7, 500000, 50),
            Ship::new("帝国歼星舰", 100000, 2000, 5, 1000000, 100),
        ]
    }

    pub fn create_galaxy(&mut self) {
        let max_distance = (self.w / 10) * (self.h / 10);
        for galaxy_name in self.galaxy.name_list.iter() {
            let mut planets = Vec::new();
            let rand = rand::thread_rng().gen_range(4, 9);
            planets.push(Planet::new("星系跳跃门", 0, 0, galaxy_name));

            //设置跳跃门舰队（星系主力舰队）
            let mut master_fleet1 = self.set_random_fleet();
            let mut master_fleet2 = self.set_random_fleet();
            master_fleet1.append(&mut master_fleet2);
            planets[0].fleet = master_fleet1;

            for i in 1..rand {
                let planet_number = rand::thread_rng().gen_range(1, 500);
                let x = rand::thread_rng().gen_range(-max_distance, max_distance);
                let y = rand::thread_rng().gen_range(-max_distance, max_distance);
                let x_distance = x - planets[0].x;
                let y_distance = y - planets[0].y;
                let distance = x_distance + y_distance;
                let plant_name = format!("开发行星{}", planet_number);
                let mut planet = Planet::new(&plant_name, x, y, galaxy_name);
                //设置市场价
                planet.goods = self.set_random_goods_price();
                planet.distance = distance;
                //设置星球护卫舰队
                planet.fleet = self.set_random_fleet();
                planets.push(planet);
            }
            self.galaxy.list.insert(galaxy_name.to_string(), planets);
        }
        // self.galaxy.current = String::from("天狼星区");
    }

    //各星球价格随机，形成差价
    fn set_random_goods_price(&self) -> Vec<Goods> {
        let mut tmp_goods = Vec::new();
        for v in self.goods.iter() {
            let mut value = v.clone();
            value.price = rand::thread_rng().gen_range(value.price >> 1, value.price);
            tmp_goods.push(value);
        }
        tmp_goods
    }

    //为各星球生成舰队
    fn set_random_fleet(&self) -> Vec<Ship> {
        let tmp_number = rand::thread_rng().gen_range(5, 10);
        let mut tmp_ships = Vec::new();
        for i in 0..tmp_number {
            let tmp_rand = rand::thread_rng().gen_range(2, self.ships.len() - 1);
            tmp_ships.push(self.ships[tmp_rand].clone());
        }
        tmp_ships
    }

    fn save() {}
}
