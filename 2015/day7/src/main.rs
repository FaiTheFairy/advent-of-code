use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let mut circuit: Circuit = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = circuit.eval_wire(&WireId("a".to_string()));
    println!("Part 1: {sol1}");

    let mut circuit: Circuit = fs::read_to_string("input.txt")?.parse()?;
    circuit
        .rules
        .insert(WireId("b".to_string()), Expr::Value(Operand::Signal(sol1)));

    let sol2 = circuit.eval_wire(&WireId("a".to_string()));
    println!("Part 2. {sol2}");

    Ok(())
}

type Signal = u16;

struct Circuit {
    rules: HashMap<WireId, Expr>,
    cache: HashMap<WireId, Signal>,
}

impl Circuit {
    fn eval_wire(&mut self, wire_id: &WireId) -> Signal {
        // check cache
        if let Some(sig) = self.cache.get(wire_id) {
            return *sig;
        }

        let expr = self.rules.get(wire_id).cloned().expect("wire not found");
        // recursively evaluate expression.
        let value = expr.eval(self);
        self.cache.insert(wire_id.clone(), value);

        value
    }
}

impl FromStr for Circuit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut rules = HashMap::with_capacity(s.lines().count());
        for line in s.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let (lhs, rhs) = line.split_once("->").context("no arrow found")?;
            let lhs: Expr = lhs.parse()?;
            let output_wire = WireId(rhs.trim().to_string());
            rules.insert(output_wire, lhs);
        }
        Ok(Self {
            rules,
            cache: HashMap::new(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct WireId(String);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operand {
    Signal(Signal),
    Wire(WireId),
}

impl Operand {
    fn eval(&self, circuit: &mut Circuit) -> Signal {
        match self {
            Operand::Signal(sig) => *sig,
            Operand::Wire(wire_id) => circuit.eval_wire(wire_id),
        }
    }
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(signal) = s.trim().parse::<Signal>() {
            Ok(Self::Signal(signal))
        } else {
            Ok(Self::Wire(WireId(s.trim().to_string())))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Value(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, u16),
    Rshift(Operand, u16),
    Not(Operand),
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match tokens.as_slice() {
            [x] => Ok(Self::Value(x.parse()?)),
            ["NOT", x] => Ok(Self::Not(x.parse()?)),
            [x, "AND", y] => Ok(Self::And(x.parse()?, y.parse()?)),
            [x, "OR", y] => Ok(Self::Or(x.parse()?, y.parse()?)),
            [x, "LSHIFT", y] => Ok(Self::Lshift(x.parse()?, y.parse()?)),
            [x, "RSHIFT", y] => Ok(Self::Rshift(x.parse()?, y.parse()?)),
            _ => bail!("unkown expression: {s}"),
        }
    }
}

impl Expr {
    fn eval(&self, circuit: &mut Circuit) -> Signal {
        match self {
            Expr::Value(op) => op.eval(circuit),
            Expr::And(a, b) => a.eval(circuit) & b.eval(circuit),
            Expr::Or(a, b) => a.eval(circuit) | b.eval(circuit),
            Expr::Lshift(a, n) => a.eval(circuit) << n,
            Expr::Rshift(a, n) => a.eval(circuit) >> n,
            Expr::Not(a) => !a.eval(circuit),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn wire(name: &str) -> WireId {
        WireId(name.to_string())
    }

    #[test]
    fn test_operand_eval_signal() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let result = Operand::Signal(123).eval(&mut circuit);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_expr_eval_value_signal() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::Value(Operand::Signal(456));
        assert_eq!(expr.eval(&mut circuit), 456);
    }

    #[test]
    fn test_expr_eval_and() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::And(Operand::Signal(123), Operand::Signal(456));
        assert_eq!(expr.eval(&mut circuit), 123 & 456);
    }

    #[test]
    fn test_expr_eval_or() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::Or(Operand::Signal(123), Operand::Signal(456));
        assert_eq!(expr.eval(&mut circuit), 123 | 456);
    }

    #[test]
    fn test_expr_eval_lshift() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::Lshift(Operand::Signal(123), 2);
        assert_eq!(expr.eval(&mut circuit), 123 << 2);
    }

    #[test]
    fn test_expr_eval_rshift() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::Rshift(Operand::Signal(456), 2);
        assert_eq!(expr.eval(&mut circuit), 456 >> 2);
    }

    #[test]
    fn test_expr_eval_not() {
        let mut circuit = Circuit {
            rules: HashMap::new(),
            cache: HashMap::new(),
        };

        let expr = Expr::Not(Operand::Signal(123));
        assert_eq!(expr.eval(&mut circuit), !123);
    }

    #[test]
    fn test_eval_wire_direct_signal() {
        let mut rules = HashMap::new();
        let cache = HashMap::new();
        rules.insert(wire("x"), Expr::Value(Operand::Signal(123)));

        let mut circuit = Circuit { rules, cache };

        assert_eq!(circuit.eval_wire(&wire("x")), 123);
    }

    #[test]
    fn test_eval_wire_reference() {
        let mut rules = HashMap::new();
        let cache = HashMap::new();
        rules.insert(wire("x"), Expr::Value(Operand::Signal(123)));
        rules.insert(wire("y"), Expr::Value(Operand::Wire(wire("x"))));

        let mut circuit = Circuit { rules, cache };

        assert_eq!(circuit.eval_wire(&wire("y")), 123);
    }

    #[test]
    fn test_sample_circuit() {
        let mut rules = HashMap::new();

        rules.insert(wire("x"), Expr::Value(Operand::Signal(123)));
        rules.insert(wire("y"), Expr::Value(Operand::Signal(456)));
        rules.insert(
            wire("d"),
            Expr::And(Operand::Wire(wire("x")), Operand::Wire(wire("y"))),
        );
        rules.insert(
            wire("e"),
            Expr::Or(Operand::Wire(wire("x")), Operand::Wire(wire("y"))),
        );
        rules.insert(wire("f"), Expr::Lshift(Operand::Wire(wire("x")), 2));
        rules.insert(wire("g"), Expr::Rshift(Operand::Wire(wire("y")), 2));
        rules.insert(wire("h"), Expr::Not(Operand::Wire(wire("x"))));
        rules.insert(wire("i"), Expr::Not(Operand::Wire(wire("y"))));

        let cache = HashMap::new();
        let mut circuit = Circuit { rules, cache };

        assert_eq!(circuit.eval_wire(&wire("d")), 72);
        assert_eq!(circuit.eval_wire(&wire("e")), 507);
        assert_eq!(circuit.eval_wire(&wire("f")), 492);
        assert_eq!(circuit.eval_wire(&wire("g")), 114);
        assert_eq!(circuit.eval_wire(&wire("h")), 65412);
        assert_eq!(circuit.eval_wire(&wire("i")), 65079);
        assert_eq!(circuit.eval_wire(&wire("x")), 123);
        assert_eq!(circuit.eval_wire(&wire("y")), 456);
    }

    #[test]
    fn test_eval_wire_populates_cache() {
        let mut rules = HashMap::new();
        rules.insert(wire("x"), Expr::Value(Operand::Signal(123)));
        rules.insert(
            wire("d"),
            Expr::And(Operand::Wire(wire("x")), Operand::Signal(456)),
        );

        let cache = HashMap::new();

        let mut circuit = Circuit { rules, cache };

        let result = circuit.eval_wire(&wire("d"));
        assert_eq!(result, 123 & 456);
    }
}
