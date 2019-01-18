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

static WAIT_TIME: u64 = 100;

pub struct Game {
    w: i32,
    h: i32,
    goods: [Goods; 9],
    ships: [Ship; 7],
    galaxy: Galaxy,
    player: Vec<Player>,
    planet_name :String,
//    planet_index:usize,
    pub docked: bool,
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
            player: vec![],
            docked:true,
//            planet_index:1,
            user_input:String::from(""),
            planet_name:String::from("")
        }
    }

    pub fn start(&mut self) {
        self.create_galaxy();
        let ship = self.ships[1].clone();
        let planet_index = 1;
        let planet = self.galaxy.get_planet(planet_index);
        self.planet_name = planet.name.clone();
        let mut player = Player::new("路漫漫", 10000, ship, planet_index,&planet.name);
        self.player.push(player);
        let mut goods = self.goods[3].clone();
        goods.quantity = 5;
        self.player[0].buy_goods(goods);
        self.process_logic();
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
                    Self::wait();
                    continue
                }
            }
        }
    }

    fn process_logic(&mut self) {
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
                    if self.docked {
                        println!("{}",Menu::docked());
                    }else{
                        println!("{}",Menu::help());
                    }
                }
                "cargo" => {
                    println!("-- 货舱 -- {}",self.player[0].cargo_inventory());
                }
                "map" => {
                    println!("-- map --{}",self.galaxy.map());
                }
                "goto" => {
                    let planets_list = self.galaxy.get_planet_list(&self.galaxy.current);
                    println!("-- {} map --{}\n选择前往地点：",&self.galaxy.current,planets_list);
                    choice = Self::get_choice();
                    //todo: out of range check
                    self.goto_planet(choice);
                }
                "state" => {
                    let mut current_planet= self.galaxy.get_planet(self.player[0].planet_index);
                    let state = self.player[0].get_state((self.docked,current_planet));
                    println!("{}",&state[..]);
                }
                "buy" if self.docked => {
                    if self.player[0].planet_index == 0 {
                        println!("当前位置没有商品!");
                        continue
                    }
                    let mut current_planet= self.galaxy.get_planet(self.player[0].planet_index);
                    let planet_goods = &current_planet.goods;
                    Menu::show(planet_goods);
                    println!("> 选择购买的物品");
                    choice = Self::get_choice();
//
                    let mut goods = planet_goods[choice].clone();
                    println!("> 需要多少的{}？", goods.name);
                    choice = Self::get_choice();
                    goods.quantity = choice as u32;
                    let info = self.player[0].buy_goods(goods);
                    println!("{}",info);
                }

                _ => {
                    Self::wait();
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
            Goods::new("金", 1200),
            Goods::new("能量护盾", 1500),
            Goods::new("矿石", 30),
            Goods::new("武器", 100),
            Goods::new("建材", 10),
            Goods::new("导弹", 300),
            Goods::new("暗物质", 450000),
            Goods::new("生活包", 500),
        ]
    }

    fn create_ship() -> [Ship; 7] {
        [
            Ship::new("采矿船", 1000, 200, 100, 100000, 2500),
            Ship::new("轻型货机", 500, 50, 300, 25000, 2000),
            Ship::new("PLA战斗机", 300, 100, 500, 75000, 2000),
            Ship::new("CR90护卫舰", 3000, 750, 300, 125000, 3000),
            Ship::new("PLA驱逐舰", 50000, 500, 200, 400000, 4000),
            Ship::new("帝国驱逐舰", 50000, 1000, 100, 500000, 5000),
            Ship::new("帝国歼星舰", 100000, 2000, 50, 1000000, 10000),
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
                let distance = (x_distance + y_distance).abs();
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

    //各星球商品价格随机，形成差价
    fn set_random_goods_price(&self) -> Vec<Goods> {
        let mut tmp_goods = Vec::new();
        for v in self.goods.iter() {
            let mut value = v.clone();
            value.price = rand::thread_rng().gen_range(value.price >> 1, value.price<<1);
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

    pub fn goto_planet(&mut self, to_planet_index:usize) {
        let distance = self.calculate_planets_distance(to_planet_index);
        if self.player[0].ship.calculate_fuel(distance) {
            Self::auto_show("已锁定目标位置，开始巡航...");
            self.player[0].day += distance as f32/self.player[0].ship.speed as f32;
            self.calculate_years();
            self.player[0].planet_index = to_planet_index;
            if to_planet_index == 0 {
                Self::auto_show("进入星系跳跃门...");
                Self::auto_show("开始跃迁...");
                self.galaxy.jump();
                println!("跃迁完成，欢迎来到 {} ", &self.galaxy.current);
            }else {
                let to_planet = self.galaxy.get_planet(to_planet_index);
                println!("已到达 {} ", to_planet.name);
            }
        }else {
            println!("燃料不够！");
        }

    }

    pub fn calculate_planets_distance(&self,to_planet_index: usize) -> i32
    {
        let current_planet = self.galaxy.get_planet(self.player[0].planet_index);
        let to_planet = self.galaxy.get_planet(to_planet_index);
        let distance = current_planet.distance - to_planet.distance;
        distance.abs()
    }

    pub fn calculate_years(&mut self)
    {
        if self.player[0].day > 365.0 {
            self.player[0].day -= 365.0;
            self.player[0].year += 1.0;
        }
    }

    pub fn auto_show (text:&str) {
        println!("{}",text);
        thread::sleep(Duration::from_millis(2000));
    }

    fn save() {}

    fn wait() {
        thread::sleep(Duration::from_millis(WAIT_TIME));
    }
}
