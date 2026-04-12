fn main() {
    let boss = Boss { hp: 71, damage: 10 };
    let game = GameState {
        boss,
        ..Default::default()
    };

    let sol1 = game.least_mana_to_win(false);
    println!("Part 1: {sol1}");

    let sol1 = game.least_mana_to_win(true);
    println!("Part 1: {sol1}");
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct GameState {
    player: Player,
    boss: Boss,
    effects: Effects,
    mana_spent: i32,
}

impl GameState {
    fn least_mana_to_win(self, hard_mode: bool) -> i32 {
        let mut best = i32::MAX;
        self.search(&mut best, hard_mode);
        best
    }

    fn search(self, best: &mut i32, hard_mode: bool) {
        if self.mana_spent >= *best {
            return;
        }

        for spell in Spell::ALL {
            if let Some(next) = self.play_round(spell, hard_mode) {
                if next.player_won() {
                    *best = (*best).min(next.mana_spent);
                } else {
                    next.search(best, hard_mode);
                }
            }
        }
    }

    fn play_round(&self, spell: Spell, hard_mode: bool) -> Option<Self> {
        let mut next = *self;

        if hard_mode {
            next.player.hp -= 1;
            if next.player_lost() {
                return None;
            }
        }

        next.apply_effects();
        if next.player_won() {
            return Some(next);
        }

        if !next.can_cast(spell) {
            return None;
        }

        next.cast_spell(spell);
        if next.player_won() {
            return Some(next);
        }

        next.apply_effects();
        if next.player_won() {
            return Some(next);
        }

        next.boss_attack();
        if next.player_lost() {
            return None;
        }

        Some(next)
    }

    fn player_won(&self) -> bool {
        self.boss.hp <= 0
    }

    fn player_lost(&self) -> bool {
        self.player.hp <= 0
    }

    fn apply_effects(&mut self) {
        if self.effects.shield > 0 {
            self.effects.shield -= 1;
        }

        if self.effects.poison > 0 {
            self.boss.hp -= 3;
            self.effects.poison -= 1;
        }

        if self.effects.recharge > 0 {
            self.player.mana += 101;
            self.effects.recharge -= 1;
        }
    }

    fn cast_spell(&mut self, spell: Spell) {
        let cost = spell.cost();
        self.player.mana -= cost;
        self.mana_spent += cost;

        match spell {
            Spell::MagicMissile => self.boss.hp -= 4,
            Spell::Drain => {
                self.boss.hp -= 2;
                self.player.hp += 2;
            }
            Spell::Shield => self.effects.shield = 6,
            Spell::Poison => self.effects.poison = 6,
            Spell::Recharge => self.effects.recharge = 5,
        }
    }

    fn can_cast(&self, spell: Spell) -> bool {
        if self.player.mana < spell.cost() {
            return false;
        }

        match spell {
            Spell::MagicMissile | Spell::Drain => true,
            Spell::Shield => self.effects.shield == 0,
            Spell::Poison => self.effects.poison == 0,
            Spell::Recharge => self.effects.recharge == 0,
        }
    }

    fn armor(&self) -> i32 {
        if self.effects.shield > 0 { 7 } else { 0 }
    }

    fn boss_attack(&mut self) {
        let damage = (self.boss.damage - self.armor()).max(1);
        self.player.hp -= damage;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Player {
    hp: i32,
    mana: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self { hp: 50, mana: 500 }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const ALL: [Spell; 5] = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ];

    fn cost(self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

/// This captures the number of turns remaining for the effect
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Effects {
    shield: i32,
    poison: i32,
    recharge: i32,
}
