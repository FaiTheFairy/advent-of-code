use std::str::FromStr;

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input: Input = std::fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input
        .solve_part_1()
        .context("no solution found for part 1")?;
    println!("Part 1: {sol1}");

    let sol2 = input
        .solve_part_2()
        .context("no solution found for part 2")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    ingredients: Vec<Ingredient>,
}

impl Input {
    fn solve_part_1(&self) -> Option<isize> {
        let mut best: Option<isize> = None;

        self.for_each_recipe(100, |recipe| {
            let score: isize = recipe.total_properties(&self.ingredients).score();

            best = Some(match best {
                Some(current) => current.max(score),
                None => score,
            });
        });

        best
    }

    fn solve_part_2(&self) -> Option<isize> {
        let mut best: Option<isize> = None;

        self.for_each_recipe(100, |recipe| {
            let properties = recipe.total_properties(&self.ingredients);

            if properties.calories == 500 {
                let score = properties.score();
                best = Some(match best {
                    Some(current) => current.max(score),
                    None => score,
                })
            }
        });

        best
    }

    /// Iterates over all possible recipes that distribute `total_teaspoons`
    /// across all ingredients.
    ///
    /// A recipe is represented as a vector of amounts (teaspoons) per ingredient,
    /// where the sum of all amounts equals `total_teaspoons`.
    ///
    /// This function does not allocate all recipes at once. Instead, it generates
    /// each recipe on the fly and passes it to the closure `f`.
    ///
    /// # Example
    /// For 3 ingredients and `total_teaspoons = 4` this will generate:
    /// [0, 0, 4], [0, 1, 3], [0, 2, 2], [0, 3, 1], [0, 4, 0],
    /// [1, 0, 3], [1, 1, 2]], ..., [4, 0, 0]
    fn for_each_recipe<F>(&self, total_teaspoons: isize, mut f: F)
    where
        F: FnMut(Recipe),
    {
        let mut amounts = vec![0; self.ingredients.len()];
        self.fill_recipes(0, total_teaspoons, &mut amounts, &mut f);
    }

    /// Recursively builds all valid recipes by assiging teaspoons to each ingredient.
    ///
    /// This function fills the `amounts` slice one index at a time:
    /// - `index` indicates which ingredient we are currently assigining
    /// - `remaining` is the number of teaspoons left to distribute
    ///
    /// For each ingredient (except the last), it tries all possible allocations
    /// from `0..=remaining`, and recursively assigns the rest.
    ///
    /// When the last ingredient is reached, it is assigned all remaining teaspoons
    /// to ensure the total sum is correct. At that point, a complete recipe is formed
    /// and passed to the closure `f`.
    fn fill_recipes<F>(&self, index: usize, remaining: isize, amounts: &mut [isize], f: &mut F)
    where
        F: FnMut(Recipe),
    {
        if index + 1 == self.ingredients.len() {
            amounts[index] = remaining;
            f(Recipe {
                amounts: amounts.to_vec(),
            });
            return;
        }

        for amount in 0..=remaining {
            amounts[index] = amount;
            self.fill_recipes(index + 1, remaining - amount, amounts, f);
        }
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ingredients = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { ingredients })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Recipe {
    amounts: Vec<isize>,
}

impl Recipe {
    fn total_properties(&self, ingredients: &[Ingredient]) -> Properties {
        ingredients
            .iter()
            .zip(self.amounts.iter().copied())
            .fold(Properties::default(), |acc, (ingredient, amount)| {
                acc + ingredient.properties_per_tsp * amount
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ingredient {
    name: String,
    properties_per_tsp: Properties,
}

impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, properties) = s.split_once(':').context("ingredient missing ':'")?;

        let name = name.trim().to_string();
        let properties = properties.parse().context("failed to parse properties")?;

        Ok(Self {
            name,
            properties_per_tsp: properties,
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Properties {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Properties {
    fn score(self) -> isize {
        [
            self.capacity.max(0),
            self.durability.max(0),
            self.flavor.max(0),
            self.texture.max(0),
        ]
        .iter()
        .product()
    }
}

impl std::ops::Add for Properties {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl std::ops::Mul<isize> for Properties {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl FromStr for Properties {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split([' ', ',']).filter(|t| !t.is_empty()).collect();

        match tokens.as_slice() {
            [
                "capacity",
                capacity,
                "durability",
                durability,
                "flavor",
                flavor,
                "texture",
                texture,
                "calories",
                calories,
            ] => {
                let capacity = capacity.parse()?;
                let durability = durability.parse()?;
                let flavor = flavor.parse()?;
                let texture = texture.parse()?;
                let calories = calories.parse()?;
                Ok(Self {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                })
            }
            _ => bail!("malformed properties: {s}"),
        }
    }
}
