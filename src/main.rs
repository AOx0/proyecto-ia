use rand::{seq::IteratorRandom, Rng};

fn main() {
    let mut n8_queens = NQueen::new(8).into_random_state();

    let mut iterations = 0;
    while n8_queens.step() != 0 {
        // println!("{:?}\n", n8_queens);
        iterations += 1;
    }
    println!("{}", iterations);
    println!("{}", n8_queens);
}

struct NQueen {
    n: usize,
    queens: Vec<usize>,

    last_queens: Vec<Vec<usize>>,
    costs: Vec<(usize, usize, usize)>,
    verbose: bool,
}

enum Side {
    Left,
    Right,
}

impl NQueen {
    fn cost_of(&self, of: usize) -> [usize; 3] {
        [
            self.column_c(of),
            self.diagonal_c(of, Side::Left),
            self.diagonal_c(of, Side::Right),
        ]
    }

    fn column_c(&self, of: usize) -> usize {
        (0..self.n)
            .filter(|&x| x != of && self.queens[x] == self.queens[of])
            .count()
    }

    fn diagonal_c(&self, of: usize, side: Side) -> usize {
        let val = self.queens[of];

        (0..self.n)
            .filter(|&x| {
                if x != of {
                    let offset = x.checked_sub(of).unwrap_or_else(|| of - x);
                    if let Some(res) = match side {
                        Side::Left => val.checked_sub(offset),
                        Side::Right => val.checked_add(offset),
                    } {
                        return self.queens[x] == res;
                    }
                }
                false
            })
            .count()
    }

    fn into_random_state(mut self) -> Self {
        self.last_queens.clear();
        self.queens.iter_mut().for_each(|queen| {
            *queen = rand::random::<usize>() % self.n;
        });
        self
    }

    fn overall_cost(&self) -> usize {
        (0..self.n)
            .map(|queen| self.cost_of(queen).into_iter().sum::<usize>())
            .sum::<usize>()
    }

    fn step(&mut self) -> usize {
        let mut rng = rand::thread_rng();

        (0..self.n).for_each(|queen| {
            self.costs[queen] = (
                queen,
                self.cost_of(queen).into_iter().sum::<usize>(),
                self.queens[queen],
            );
        });
        self.costs.sort_by(|a, b| a.1.cmp(&b.1));

        let worst_value = self.costs.last().and_then(|&x| Some(x.1)).unwrap();
        let (worst_pos, _, prev_val) = self
            .costs
            .iter()
            .filter(|&x| x.1 == worst_value)
            .choose(&mut rng)
            .copied()
            .unwrap();

        (0..self.n).filter(|&col| col != prev_val).for_each(|col| {
            self.costs[col] = (
                col,
                {
                    self.queens[worst_pos] = col;
                    let res = self.cost_of(worst_pos).iter().sum::<usize>();
                    self.queens[worst_pos] = prev_val;
                    res
                },
                0,
            )
        });
        self.costs.sort_by(|a, b| a.1.cmp(&b.1));

        let (_, best_cost, _) = self.costs[0];
        let (new_cost, _, _) = self
            .costs
            .iter()
            .filter(|&x| x.1 == best_cost)
            .choose(&mut rng)
            .copied()
            .unwrap();

        // Si hay colisión se trata de un camino sin salida. Forzamos algo de aleatoriedad
        if self.last_queens.contains(&self.queens) {
            self.queens[rng.gen_range(0..self.n)] = rng.gen_range(0..self.n);
        } else {
            self.last_queens.push(self.queens.clone())
        }

        self.queens[worst_pos] = new_cost;
        self.overall_cost()
    }

    fn new(with_n: usize) -> Self {
        NQueen {
            n: with_n,
            queens: vec![0; with_n],
            last_queens: Vec::with_capacity(100),
            costs: vec![(0, 0, 0); with_n],
            verbose: false,
        }
    }
}

impl std::fmt::Display for NQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.n).try_for_each(|row| {
            (0..self.n)
                .map(|i| if self.queens[row] == i { '*' } else { '.' })
                .try_for_each(|val| write!(f, "{} ", val))?;

            if self.verbose {
                let cost = self.cost_of(row);
                let [lval, rval, cval] = { [cost[0], cost[1], cost[2]] };
                write!(
                    f,
                    " | ld:{lval:>2} rd:{rval:>2} cc:{cval:>2} | tt:{:>2}",
                    cost.iter().sum::<usize>()
                )?;
            }
            if row != self.n - 1 {
                write!(f, "\n")?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
