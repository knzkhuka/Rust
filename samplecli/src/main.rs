use std::{fs::File, io::{stdin, BufRead, BufReader}, path::PathBuf};

use anyhow::{bail, ensure, Context, Result};
use clap::Clap;

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(2 * 2, 4);
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("10").unwrap(), 10);
        assert_eq!(calc.eval("-1").unwrap(), -1);
        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("1 1 ^").is_err());
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}

struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    pub fn eval(&self, fomula: &str) -> Result<i32> {
        let mut tokens = fomula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
}

#[derive(Clap, Debug)]
#[clap(
    name = "intoduction to Rust programming RPN",
    version = "1.0.0",
    author = "knzkhuka",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}
