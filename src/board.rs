use fuda::{Fuda, Gara, Month, TanColor};

pub struct Board {
    yama: Vec<Fuda>,      // 山札
    ba: Vec<Fuda>,        // 場
    te_oya: Vec<Fuda>,    // 手札 親
    mochi_oya: Vec<Fuda>, // 獲得 親
    te_ko: Vec<Fuda>,     // 手札 子
    mochi_ko: Vec<Fuda>,  // 獲得 子
}

macro_rules! set_yama {
    ($i:ident, $m:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr) => {
        $i.push(Fuda::new($m, $x1));
        $i.push(Fuda::new($m, $x2));
        $i.push(Fuda::new($m, $x3));
        $i.push(Fuda::new($m, $x4));
    };
}

impl Board {
    pub fn new() -> Board {
        let mut yama = Vec::<Fuda>::new();
        set_yama!(
            yama,
            Month::Jan,   // 松
            Gara::Hikari, // 鶴
            Gara::Tan(TanColor::Aka),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Feb, // 梅
            Gara::Tane, // 鶯
            Gara::Tan(TanColor::Aka),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Mar,   // 桜
            Gara::Hikari, // 幕
            Gara::Tan(TanColor::Aka),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Apr, // 藤
            Gara::Tane, // 不如帰
            Gara::Tan(TanColor::Tan),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::May, // 菖蒲
            Gara::Tane, // 八ツ橋
            Gara::Tan(TanColor::Tan),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Jun, // 牡丹
            Gara::Tane, // 蝶
            Gara::Tan(TanColor::Ao),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Jul, // 萩
            Gara::Tane, // 猪
            Gara::Tan(TanColor::Tan),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Aug,   // 芒
            Gara::Hikari, // 月
            Gara::Tane,   // 雁
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Sep, // 菊
            Gara::Tane, // 鹿
            Gara::Tan(TanColor::Ao),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Oct, // 紅葉
            Gara::Tane, // 鹿
            Gara::Tan(TanColor::Ao),
            Gara::Kasu,
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Nov,   // 柳
            Gara::Hikari, // 小野道風
            Gara::Tane,   // 燕
            Gara::Tan(TanColor::Ao),
            Gara::Kasu
        );
        set_yama!(
            yama,
            Month::Dec,   // 桐
            Gara::Hikari, // 鳳凰
            Gara::Kasu,
            Gara::Kasu,
            Gara::Kasu
        );

        Board {
            yama: yama,
            ba: Vec::new(),
            te_oya: Vec::new(),
            mochi_oya: Vec::new(),
            te_ko: Vec::new(),
            mochi_ko: Vec::new(),
        }
    }
}
