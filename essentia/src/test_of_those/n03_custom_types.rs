#[test]
fn structures() {
    #[derive(Debug)]
    struct PlayerStatus {
        position_xy: (i32, i32),
        maxhp: u32,
        maxmp: u32,
        hitpoint: u32,
        manapoint: u32,
        alive_situation: bool,
    }
    impl PlayerStatus {
        fn default_unit_with_xy(x: i32, y: i32) -> PlayerStatus {
            PlayerStatus {
                position_xy: (x, y),
                maxhp: 100,
                maxmp: 30,
                hitpoint: 100,
                manapoint: 30,
                alive_situation: true,
            }
        }

        fn display_status(&self) {
            println!(
                "position: ({}, {})\nhp: {}/{}\nmp: {}/{}\nalive: {}",
                self.position_xy.0,
                self.position_xy.1,
                self.hitpoint,
                self.maxhp,
                self.manapoint,
                self.maxmp,
                self.alive_situation
            );
        }

        fn move_to_another_xy_position(&mut self, x: i32, y: i32) {
            self.position_xy = (x, y);
        }

        fn get_hit(&mut self, damage: u32) {
            if damage >= self.hitpoint {
                self.hitpoint = 0;
                self.alive_situation = false;
            } else {
                self.hitpoint -= damage
            }
        }

        fn heal_itself(&mut self) {
            if self.manapoint >= 5 {
                self.manapoint -= 5;
                if (self.hitpoint + 20) > self.maxhp {
                    self.hitpoint = self.maxhp
                } else {
                    self.hitpoint += 20
                }
            }
        }
    }

    let mut tom = PlayerStatus::default_unit_with_xy(0, 0);
    tom.get_hit(75);
    tom.move_to_another_xy_position(10, 20);
    tom.heal_itself();
    tom.display_status();
}

#[test]
fn enums() {
    enum YoodyCandyItem {
        DarkChocolate = 114201,
        MilkChocolate = 114202,
        LemonPop = 136651,
        PineappleCandy = 136631,
    }
    impl YoodyCandyItem {
        fn into_string(&self) -> String {
            match self {
                YoodyCandyItem::DarkChocolate => "DarkChocolate".into(),
                YoodyCandyItem::MilkChocolate => "MilkChocolate".into(),
                YoodyCandyItem::LemonPop => "LemonPop".into(),
                YoodyCandyItem::PineappleCandy => "PineappleCandy".into(),
            }
        }

        fn candy_bar_code(&self) -> String {
            match self {
                YoodyCandyItem::DarkChocolate => "114201".into(),
                YoodyCandyItem::MilkChocolate => "114202".into(),
                YoodyCandyItem::LemonPop => "136651".into(),
                YoodyCandyItem::PineappleCandy => "136631".into(),
            }
        }
    }

    let _ = YoodyCandyItem::DarkChocolate.candy_bar_code();
    let _ = YoodyCandyItem::MilkChocolate.into_string();
    let _ = YoodyCandyItem::LemonPop as i32;
    let _ = YoodyCandyItem::PineappleCandy as u32;

    enum Klist {
        Note(u32, Box<Klist>),
        Nil,
    }
    impl Klist {
        fn new() -> Klist {
            Klist::Nil
        }

        fn prepend(self, begin_u32: u32) -> Klist {
            Klist::Note(begin_u32, Box::new(self))
        }

        fn len(&self) -> u64 {
            match *self {
                Self::Nil => 0,
                Self::Note(_, ref tail) => 1 + tail.len(),
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Self::Note(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                Self::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    let mut klist_a = Klist::new();
    klist_a = klist_a.prepend(10);
    klist_a = klist_a.prepend(15);
    klist_a = klist_a.prepend(20);

    dbg!(klist_a.len());
    dbg!(klist_a.stringify());
}

#[test]
fn constants() {
    static MESSAGE: &str = "A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.";
    const PI_SQURE: f64 = std::f64::consts::PI * std::f64::consts::PI;

    dbg!(MESSAGE);
    dbg!(PI_SQURE);
}
