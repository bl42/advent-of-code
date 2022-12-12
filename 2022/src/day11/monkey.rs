use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Test {
    Divisible(f32),
}

#[derive(Debug, PartialEq, Clone)]
struct TestResult {
    success: usize,
    failure: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Monkey {
    pub items: Vec<i128>,
    operation: String,
    pub test: Test,
    test_result: TestResult,
    pub inspect_count: i32,
}

impl Monkey {
    fn inspect_items(&self) -> Vec<i128> {
        self.items.iter().map(|i| self.increase_item(i)).collect()
    }
    pub fn throw_item(&mut self, relief: Option<f64>) -> Option<(usize, i128)> {
        let Some(item) = self.items.pop() else {
     		return None;
      	};

        let increased = self.increase_item(&item) as f64;
        self.increase_count();

        let decreased = match relief {
            Some(i) => increased % i,
            None => f64::floor(increased / 3.0),
        };

        let test_applied = match self.test {
            Test::Divisible(n) => decreased / n as f64,
        };

        let direction = if test_applied.fract() == 0.0 {
            self.test_result.success
        } else {
            self.test_result.failure
        };

        Some((direction, decreased as i128))
    }
    fn increase_count(&mut self) {
        self.inspect_count += 1;
    }
    fn increase_item(&self, item: &i128) -> i128 {
        let body = self.operation.as_str().split(' ').collect::<Vec<&str>>();
        let left = body[0];
        let operator = body[1];
        let right = body[2].parse::<i128>();

        match (left, operator, right) {
            ("old", "*", Err(_)) => item * item,
            ("old", "+", Err(_)) => item + item,
            ("old", "*", Ok(right)) => item * right,
            ("old", "+", Ok(right)) => item + right,
            ("old", "-", Ok(right)) => item - right,
            ("old", "/", Ok(right)) => item / right,
            _ => unreachable!(""),
        }
    }
}

impl FromStr for Monkey {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<(&str, &str)> = s
            .lines()
            .map(|line| line.split_once(':').unwrap())
            .collect();

        let mut items: Vec<i128> = Vec::new();
        let mut operation = String::new();
        let mut test = Test::Divisible(1.0);
        let mut test_result = TestResult {
            success: 0,
            failure: 0,
        };

        for (key, value) in &lines {
            let k = key.trim();
            if k == "Starting items" {
                items = value
                    .trim()
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
            }

            if k == "Operation" {
                let (_, fnc) = value.split_once("= ").unwrap();
                operation = fnc.to_owned();
            }
            if k == "Test" {
                test = match value.trim().split_once(" by ").unwrap() {
                    ("divisible", num) => Test::Divisible(num.parse::<f32>().unwrap()),
                    _ => unreachable!(""),
                }
            }
            if k == "If true" {
                test_result.success = value
                    .split_once("monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse::<usize>()
                    .unwrap();
            }
            if k == "If false" {
                test_result.failure = value
                    .split_once("monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse::<usize>()
                    .unwrap();
            }
        }

        Ok(Monkey {
            items,
            operation,
            test,
            test_result,
            inspect_count: 0,
        })
    }
}
