fn main() {
    println!(
        "{}",
        get_order("milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza".to_string())
    );
}

/// Some new cashiers started to work at your restaurant.
///
/// They are good at taking orders, but they don't know how to capitalize words, or use a space bar!
///
/// All the orders they create look something like this:
///
/// "milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza"
///
/// The kitchen staff are threatening to quit, because of how difficult it is to read the orders.
///
/// Their preference is to get the orders as a nice clean string with spaces and capitals like so:
///
/// "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke"
///
/// The kitchen staff expect the items to be in the same order as they appear in the menu.
///
/// The menu items are fairly simple, there is no overlap in the names of the items:
///
/// 1. Burger
/// 2. Fries
/// 3. Chicken
/// 4. Pizza
/// 5. Sandwich
/// 6. Onionrings
/// 7. Milkshake
/// 8. Coke
pub fn get_order(input: String) -> String {
    let cap = input.len() + input.len() / 4;
    let mut res = String::with_capacity(cap);

    let mut p = 0usize;
    let mut m = 0;
    let mut n = 0;
    let mut f = 0;
    let mut g = 0;
    let mut o = 0;
    let mut u = 0;
    let mut w = 0;

    for b in input.bytes() {
        match b {
            b'p' => p += 1,
            b'm' => m += 1,
            b'n' => n += 1,
            b'f' => f += 1,
            b'g' => g += 1,
            b'o' => o += 1,
            b'u' => u += 1,
            b'w' => w += 1,
            _ => {}
        }
    }

    for _ in 0..u {
        res.push_str("Burger ");
    }
    for _ in 0..f {
        res.push_str("Fries ");
    }
    for _ in 0..n - 3 * (g - u) - w {
        res.push_str("Chicken ");
    }
    for _ in 0..p {
        res.push_str("Pizza ");
    }
    for _ in 0..w {
        res.push_str("Sandwich ");
    }
    for _ in 0..g - u {
        res.push_str("Onionrings ");
    }
    for _ in 0..m {
        res.push_str("Milkshake ");
    }
    for _ in 0..o - 2 * (g - u) {
        res.push_str("Coke ");
    }
    res.pop();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            get_order(
                "milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza".to_string()
            ),
            "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            get_order("pizzachickenfriesburgercokemilkshakefriessandwich".to_string()),
            "Burger Fries Fries Chicken Pizza Sandwich Milkshake Coke".to_string()
        );
    }
}
