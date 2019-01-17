pub struct Menu;
impl Menu
{
    pub fn start() -> String {
        let arr = [
            "--void fleet--",
            "1 开始",
            "2 载入",
            "3 帮助"
        ];
        Self::stringify(&arr)
    }

    fn stringify(arr:&[&str]) -> String {
        let mut menu_str = String::new();
        for v in arr.into_iter() {
            menu_str.push_str("\n");
            menu_str.push_str(v);
        }
        menu_str
    }

    pub fn help() -> String {
        let arr = ["--巡航状态帮助菜单--",
            "map - 显示地图信息",
            "goto - 选择前往星球",
            "jump - 仅在星系跳跃大门可用，进入其他星系",
            "dock - 停靠附近星球空间站",
            "cargo - 货仓物品列表",
            "state - 显示当前状态",
            "data - 显示星球资料",
            "save - 保存游戏",
            "load - 载入游戏",
            "load - 载入游戏",
            "quit - 退出"];
        Self::stringify(&arr)
    }


    pub fn market() -> String {
        let arr = ["--市场--",
        "1 买",
        "2 卖",
        "3 取消"];
        Self::stringify(&arr)
    }


    pub fn docked() -> String {
        let arr = ["--空间站内帮助菜单--",
        "buy - 购买商品",
        "sell - 出售商品",
        "repair - 修理",
        "refuel - 加燃料",
        "ship - 购买飞船",
        "upgrades - 升级飞船",
        "simulator - 模拟战斗测试",
        "undock - 离开"];
        Self::stringify(&arr)
    }

    pub fn navigation() -> String {
        let arr = ["--导航菜单--",
        "1 本地系统",
        "2 星系地图"];
        Self::stringify(&arr)
    }

    pub fn menu_galaxy_map() -> String {
        let arr = ["---星系地图---",
        "* 人马座",
        "|",
        "* 烈阳星区",
        "|",
        "* 天狼星区",
        "|",
        "* 北落师门",
        "|",
        "* PLA",
        "|",
        "* 北极星区"];
        Self::stringify(&arr)
    }





}