use std::fmt::{Debug, Display};

fn main() {
    let mut n8_queens = NQueen::<20>::default().into_random_state();

    while n8_queens.step() != 0 {
        println!("{:?}", n8_queens);
    }
    println!("{:?}", n8_queens);
}

struct NQueen<const N: usize> {
    queens: [usize; N],
}

enum Side {
    Left,
    Right,
}

impl<const N: usize> NQueen<N> {
    fn cost_of(&self, of: usize) -> [usize; 3] {
        [
            self.column_c(of),
            self.diagonal_c(of, Side::Left),
            self.diagonal_c(of, Side::Right),
        ]
    }

    fn column_c(&self, of: usize) -> usize {
        (0..N)
            .filter(|&x| x != of && self.queens[x] == self.queens[of])
            .count()
    }

    fn diagonal_c(&self, of: usize, side: Side) -> usize {
        let val = self.queens[of];

        (0..N)
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

    fn into_random_state(self) -> Self {
        let mut res = Self::default();
        res.queens.iter_mut().for_each(|queen| {
            *queen = rand::random::<usize>() % N;
        });
        res
    }

    fn overall_cost(&self) -> usize {
        (0..N)
            .map(|queen| self.cost_of(queen).into_iter().sum::<usize>())
            .sum::<usize>()
    }

    fn step(&mut self) -> usize {
        let mut costs = (0..N)
            .map(|queen| {
                (
                    queen,
                    self.cost_of(queen).into_iter().sum::<usize>(),
                    self.queens[queen],
                )
            })
            .collect::<Vec<_>>();

        costs.sort_by(|a, b| a.1.cmp(&b.1));

        let most_exp = costs.last().copied().unwrap();
        costs = costs
            .into_iter()
            .filter(|&x| x.1 == most_exp.1)
            .collect::<Vec<_>>();
        let most_exp_idx = rand::random::<usize>() % costs.len();
        let most_exp: (usize, usize, usize) = costs[most_exp_idx];

        let mut new_costs = (0..N)
            .map(|col| {
                (col, {
                    self.queens[most_exp.0] = col;
                    self.cost_of(most_exp.0).iter().sum::<usize>()
                })
            })
            .filter(|&x| x.0 != most_exp.2)
            .collect::<Vec<_>>();
        new_costs.sort_by(|a, b| a.1.cmp(&b.1));

        let best_cost = new_costs.first().copied().unwrap();
        new_costs = new_costs
            .into_iter()
            .filter(|&x| x.1 == best_cost.1)
            .collect::<Vec<_>>();
        let new_cost_idx = rand::random::<usize>() % new_costs.len();
        let new_cost = new_costs[new_cost_idx];

        // println!("{:?}", costs);
        // println!("{:?}", most_exp);
        // println!("{:?}", new_costs);
        // println!("{:?}", self.overall_cost());
        self.queens[most_exp.0] = new_cost.0;
        // println!("{:?}", self.overall_cost());
        self.overall_cost()
    }
}

impl<const N: usize> Default for NQueen<N> {
    fn default() -> Self {
        NQueen { queens: [0; N] }
    }
}

impl<const N: usize> Display for NQueen<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..N).try_for_each(|row| {
            (0..N)
                .map(|i| if self.queens[row] == i { '*' } else { '.' })
                .try_for_each(|val| write!(f, "{} ", val))?;
            if row != N - 1 {
                write!(f, "\n")?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl<const N: usize> Debug for NQueen<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..N).try_for_each(|row| {
            (0..N)
                .map(|i| if self.queens[row] == i { '*' } else { '.' })
                .try_for_each(|val| write!(f, "{} ", val))?;

            let cost = self.cost_of(row);
            let [lval, rval, cval] = { [cost[0], cost[1], cost[2]] };
            write!(
                f,
                " | ld:{lval:>2} rd:{rval:>2} cc:{cval:>2} | tt:{:>2}",
                cost.iter().sum::<usize>()
            )?;
            if row != N - 1 {
                write!(f, "\n")?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
