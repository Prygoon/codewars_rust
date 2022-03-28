fn main() {
    let coffee = CoffeeBuilder::new()
        .set_black_coffee()
        .with_sugar("Regular".to_string())
        .with_milk(3.2)
        .build();

    println!("{:?}", coffee)
}

/// You have a simple Cafeteria.
///
/// You can create 3 coffee recipes:
///
/// Black = Black coffee
/// Cubano = Cubano coffee + Brown sugar
/// Americano = Americano coffee + Milk with 0.5% fat
///
/// And you can add a lot of extra sugar and extra milk in any coffee, e.g.:
///
/// Black + Milk with 3.2% fat + Brown sugar
/// Cubano + Brown sugar + Brown sugar + Regular sugar
/// Americano + Milk with 3.2% fat + Milk with 0% fat + Regular sugar
///
/// You need to create a Coffee by implementing a CoffeeBuilder struct/class like in the Builder design pattern.
///
mod preloaded;
use preloaded::{Coffee, Milk, Sugar};

struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: String::new(),
            milk: vec![],
            sugar: vec![],
        }
    }

    fn set_black_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Black".into();
        self
    }

    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Cubano".into();
        self.sugar.push(Sugar {
            sort: "Brown".into(),
        });
        self
    }

    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Americano".into();
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk { fat });
        self
    }

    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        self.sugar.push(Sugar { sort });
        self
    }

    fn build(self) -> Coffee {
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        let coffee = CoffeeBuilder::new()
            .set_black_coffee()
            .with_sugar("Regular".to_string())
            .with_milk(3.2)
            .build();
        assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Black\", milk: [Milk { fat: 3.2 }], sugar: [Sugar { sort: \"Regular\" }] }");

        let coffee = CoffeeBuilder::new().set_antoccino_coffee().build();
        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }"
        );

        let coffee = CoffeeBuilder::new().set_cubano_coffee().build();
        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Cubano\", milk: [], sugar: [Sugar { sort: \"Brown\" }] }"
        );
    }
}
