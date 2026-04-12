#![allow(dead_code)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let boss: Fighter = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = solve_part_1(boss).context("no soln foun for part 1")?;
    println!("Part 1: {sol1}");

    let sol2 = solve_part_2(boss).context("no soln foun for part 2")?;
    println!("Part 2: {sol2}");

    Ok(())
}

fn solve_part_1(boss: Fighter) -> Option<i32> {
    Loadout::all_loadouts()
        .into_iter()
        .filter(|loadout| loadout.fighter(100).wins(boss))
        .map(|loadout| loadout.total_cost())
        .min()
}

fn solve_part_2(boss: Fighter) -> Option<i32> {
    Loadout::all_loadouts()
        .into_iter()
        .filter(|loadout| !loadout.fighter(100).wins(boss))
        .map(|loadout| loadout.total_cost())
        .max()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Fighter {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Fighter {
    fn is_alive(self) -> bool {
        self.hp > 0
    }

    fn is_dead(self) -> bool {
        self.hp <= 0
    }

    fn damage_per_turn(self, defender: Fighter) -> i32 {
        (self.damage - defender.armor).max(1)
    }

    fn turns_to_kill(self, defender: Fighter) -> i32 {
        let damage = self.damage_per_turn(defender);
        (defender.hp + damage - 1) / damage
    }

    fn wins(self, boss: Fighter) -> bool {
        self.turns_to_kill(boss) <= boss.turns_to_kill(self)
    }

    fn attack(self, defender: &mut Fighter) {
        defender.hp = (defender.hp - self.damage_per_turn(*defender)).max(0);
    }
}

impl FromStr for Fighter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut hp = None;
        let mut damage = None;
        let mut armor = None;

        for line in s.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let (key, value) = line.split_once(':').context("invalid fighter stat line")?;
            let value: i32 = value.trim().parse()?;

            match key {
                "Hit Points" => hp = Some(value),
                "Damage" => damage = Some(value),
                "Armor" => armor = Some(value),
                _ => bail!("unknown stat: {key}"),
            }
        }

        Ok(Self {
            hp: hp.context("missing Hit Points")?,
            damage: damage.context("missing Damage")?,
            armor: armor.context("missing Armor")?,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Item {
    name: &'static str,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring_1: Option<Item>,
    ring_2: Option<Item>,
}

impl Loadout {
    fn new(weapon: Item) -> Self {
        Self {
            weapon,
            armor: None,
            ring_1: None,
            ring_2: None,
        }
    }

    fn fighter(self, hp: i32) -> Fighter {
        Fighter {
            hp,
            damage: self.total_damage(),
            armor: self.total_armor(),
        }
    }

    fn total_damage(&self) -> i32 {
        self.weapon.damage
            + self.armor.map(|a| a.damage).unwrap_or(0)
            + self.ring_1.map(|a| a.damage).unwrap_or(0)
            + self.ring_2.map(|a| a.damage).unwrap_or(0)
    }

    fn total_armor(&self) -> i32 {
        self.weapon.armor
            + self.armor.map(|a| a.armor).unwrap_or(0)
            + self.ring_1.map(|a| a.armor).unwrap_or(0)
            + self.ring_2.map(|a| a.armor).unwrap_or(0)
    }

    fn total_cost(&self) -> i32 {
        self.weapon.cost
            + self.armor.map(|a| a.cost).unwrap_or(0)
            + self.ring_1.map(|a| a.cost).unwrap_or(0)
            + self.ring_2.map(|a| a.cost).unwrap_or(0)
    }

    fn all_loadouts() -> Vec<Loadout> {
        let mut out = Vec::new();

        for weapon in WEAPONS {
            out.push(Loadout::new(weapon));

            for armor in ARMOR {
                out.push(Loadout {
                    weapon,
                    armor: Some(armor),
                    ring_1: None,
                    ring_2: None,
                });
            }

            for (i, ring_1) in RINGS.into_iter().enumerate() {
                out.push(Loadout {
                    weapon,
                    armor: None,
                    ring_1: Some(ring_1),
                    ring_2: None,
                });

                for armor in ARMOR {
                    out.push(Loadout {
                        weapon,
                        armor: Some(armor),
                        ring_1: Some(ring_1),
                        ring_2: None,
                    });
                }

                for ring_2 in RINGS.into_iter().skip(i + 1) {
                    out.push(Loadout {
                        weapon,
                        armor: None,
                        ring_1: Some(ring_1),
                        ring_2: Some(ring_2),
                    });

                    for armor in ARMOR {
                        out.push(Loadout {
                            weapon,
                            armor: Some(armor),
                            ring_1: Some(ring_1),
                            ring_2: Some(ring_2),
                        });
                    }
                }
            }
        }

        out
    }
}

#[rustfmt::skip]
const WEAPONS: [Item; 5] = [
    Item { name: "Dagger",     cost: 8,  damage: 4, armor: 0 },
    Item { name: "Shortsword", cost: 10, damage: 5, armor: 0 },
    Item { name: "Warhammer",  cost: 25, damage: 6, armor: 0 },
    Item { name: "Longsword",  cost: 40, damage: 7, armor: 0 },
    Item { name: "Greataxe",   cost: 74, damage: 8, armor: 0 },
];

#[rustfmt::skip]
const ARMOR: [Item; 5] = [
    Item { name: "Leather",    cost: 13,  damage: 0, armor: 1 },
    Item { name: "Chainmail",  cost: 31,  damage: 0, armor: 2 },
    Item { name: "Splintmail", cost: 53,  damage: 0, armor: 3 },
    Item { name: "Bandedmail", cost: 75,  damage: 0, armor: 4 },
    Item { name: "Platemail",  cost: 102, damage: 0, armor: 5 },
];

#[rustfmt::skip]
const RINGS: [Item; 6] = [
    Item { name: "Damage +1",  cost: 25,  damage: 1, armor: 0 },
    Item { name: "Damage +2",  cost: 50,  damage: 2, armor: 0 },
    Item { name: "Damage +3",  cost: 100, damage: 3, armor: 0 },
    Item { name: "Defense +1", cost: 20,  damage: 0, armor: 1 },
    Item { name: "Defense +2", cost: 40,  damage: 0, armor: 2 },
    Item { name: "Defense +3", cost: 80,  damage: 0, armor: 3 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_loadouts_count() {
        // 5 weapon choices, 6 armor choices (none + 5),
        // 22 ring choices (none, 6 one ring, 15 combination ring)
        // legal loadouts should be 5 * 6 * 22 = 660
        assert_eq!(Loadout::all_loadouts().len(), 660);
    }

    #[test]
    fn test_example_fight_analytic() {
        let player = Fighter {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let boss = Fighter {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert!(player.wins(boss));
    }

    #[test]
    fn test_example_fight() {
        let mut player = Fighter {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let mut boss = Fighter {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        while player.is_alive() && boss.is_alive() {
            player.attack(&mut boss);

            if boss.is_dead() {
                break;
            }

            boss.attack(&mut player);
        }

        assert_eq!(player.hp, 2);
        assert!(player.is_alive());
        assert_eq!(boss.hp, 0);
        assert!(boss.is_dead());
    }
}
