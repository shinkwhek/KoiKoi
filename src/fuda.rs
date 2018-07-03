#[derive(Clone, PartialEq, Debug)]
pub struct Fuda {
    month: Month,
    gara: Gara,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Month {
    Jan, // 松に鶴
    Feb, // 梅にうぐいす
    Mar, // 桜に幕
    Apr, // 藤にほととぎす
    May, // 菖蒲に八ツ橋
    Jun, // 牡丹に蝶
    Jul, // 萩に猪
    Aug, // 芒に月・雁
    Sep, // 菊に盃
    Oct, // 紅葉に鹿
    Nov, // 尾道風に蛙・柳に燕
    Dec, // 桐に鳳凰
}

#[derive(Clone, PartialEq, Debug)]
pub enum Gara {
    // 絵柄
    Hikari,        // 光
    Tane,          // 種
    Tan(TanColor), // 短冊
    Kasu,          // カス
}

#[derive(Clone, PartialEq, Debug)]
pub enum TanColor {
    Aka, // Red
    Ao,  // Blue
}

impl Fuda {
    pub fn new(month: Month, gara: Gara) -> Fuda {
        Fuda {
            month: month,
            gara: gara,
        }
    }
}
