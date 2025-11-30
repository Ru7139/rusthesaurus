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
fn enums() {}

#[test]
fn constants() {}
