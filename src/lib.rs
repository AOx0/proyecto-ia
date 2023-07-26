use rand::{seq::IteratorRandom, Rng};
use std::collections::HashSet;

/// Máquina de estado de un problema de N-Reinas
///
/// Almacena el estado actual del problema así como los
/// estados vistos en el pasado.
#[derive(Clone)]
pub struct NQueens {
    n: usize,
    queens: Vec<usize>,

    last_queens: HashSet<Vec<usize>>,
    costs: Vec<(usize, usize, usize)>,
    verbose: bool,
}

enum Side {
    Left,
    Right,
}

impl NQueens {
    /// Calcula los tres aspectos que influyen en el costo total de una reina.
    ///
    /// Obtiene el costo de la reina especificada, devuelve un arreglo con
    /// 3 valores correspondientes a el costo por columnas y por las diagonales
    /// tanto a la derecha como a la izquierda.
    ///
    /// of: Número de la reina de la cual calcular el costo
    fn cost_of(&self, of: usize) -> [usize; 3] {
        [
            self.column_c(of),
            self.diagonal_c(of, Side::Left),
            self.diagonal_c(of, Side::Right),
        ]
    }

    /// Calcula el numero de reinas en la misma columna.
    ///
    /// Para hacer el cálculo cuenta el número de reinas en el vector
    /// `self.queens` que tienen el mismo valor que la reina seleccionada.
    ///
    /// Esto es posible ya que cada indice en el vector es el número de la reina y la fila donde
    /// está colocada, y el valor en cada indice es la columna donde está la reina.
    ///
    /// Ver la Sección 2.1.1 del reporte para más información.
    fn column_c(&self, of: usize) -> usize {
        (0..self.n)
            .filter(|&x| x != of && self.queens[x] == self.queens[of])
            .count()
    }

    /// Calcula el numero de reinas en la misma diagonal.
    ///
    /// Para hacerlo obtenemos la distancia de la fila de cada reina con la
    /// fila reina seleccionada. Este valor, sumado/restado al valor de la columna
    /// donde esta posicionada la reina actual, representa el número a buscar en
    /// el vector `self.queens`.
    ///
    /// Para evitar duplicidad de código unimos la busqueda de ambos lados en diagonal
    /// en una misma función, el cálculo se hace en base al lado especificado en `side`.
    fn diagonal_c(&self, of: usize, side: Side) -> usize {
        (0..self.n)
            .filter(|&x| {
                if x != of {
                    // La distancia de la reina actual a la reina seleccionada
                    #[allow(clippy::unnecessary_lazy_evaluations)]
                    let offset = x.checked_sub(of).unwrap_or_else(|| of - x);
                    // Restamos o sumamos para calcular el valor a buscar en self.queens
                    if let Some(res) = match side {
                        Side::Left => self.queens[of].checked_sub(offset),
                        Side::Right => self.queens[of].checked_add(offset),
                    } {
                        // Si es igual al valor calculado devolvemos true,
                        // lo que incrementa el contador de reinas en la misma diagonal
                        return self.queens[x] == res;
                    }
                }
                false
            })
            .count()
    }

    /// Genera un estado aleatorio incial.
    ///
    /// Destruye la instancia dada y devuelve una nueva
    /// con todos los valores en 0 y un estado aleatorio de posiciones
    /// de reinas.
    pub fn into_random_state(mut self) -> Self {
        self.last_queens.clear();
        self.queens.iter_mut().for_each(|queen| {
            *queen = rand::random::<usize>() % self.n;
        });
        self
    }

    /// Asigna si se desea o no mostrar información extra.
    pub fn with_verbose(mut self, value: bool) -> Self {
        self.verbose = value;
        self
    }

    /// Asigna un estado inicial.
    pub fn with_state(mut self, state: &[usize]) -> Option<Self> {
        (state.len() == self.queens.capacity()).then(|| {
            self.last_queens.clear();
            self.queens
                .iter_mut()
                .zip(state.iter())
                .for_each(|(q, &s)| *q = s);
            self
        })
    }

    /// Calcula el costo de todo el conjunto de reinas del tablero.
    ///
    /// Para lograrlo calculamos la suma de los 3 factores del costo para cada
    /// reina usando `cost_of` y después sumamos los resultados de todas las reinas.
    fn overall_cost(&self) -> usize {
        (0..self.n)
            .map(|queen| self.cost_of(queen).into_iter().sum::<usize>())
            .sum::<usize>()
    }

    /// Calcula el siguiente estado del tablero
    /// Devuelve el costo del nuevo estado
    pub fn step(&mut self) -> usize {
        let mut rng = rand::thread_rng();

        // Obtenemos el costo de cada una de las reinas en el estado actual
        (0..self.n).for_each(|queen| {
            self.costs[queen] = (
                queen,
                self.cost_of(queen).into_iter().sum::<usize>(),
                self.queens[queen],
            );
        });
        // Ordenamos, queremos obtener la reina con mayor costo
        self.costs.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        // Obtenemos la reina más cara
        let worst_value = self.costs.last().map(|&x| x.1).unwrap();

        // Escogemos una reina aleatoria de entre las que son igual de caras
        // que la reina más cara
        let (worst_pos, _, prev_val) = self
            .costs
            .iter()
            .filter(|&x| x.1 == worst_value)
            .choose(&mut rng)
            .copied()
            .unwrap();

        // Ahora vamos a cambiar la posición de la reina que más costo tiene
        // para reducir su costo.
        //
        // Comenzamos probando y calculando el costo de mover la reina a todas las posiciones
        // del 0 a N
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
        // Ordenamos para encontrar la nueva posición de la peor reina que mejora más
        // su costo debido a que lo reduce
        self.costs.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        // Obtenemos el valor de la nueva posible posicion para la reina.
        let (_, best_cost, _) = self.costs[0];
        // Escogemos aleatoriamente entre cualquiera de los posibles valores
        // que reducen el costo de la peor reina de igual manera que el mejor valor.
        let (new_cost, _, _) = self
            .costs
            .iter()
            .filter(|&x| x.1 == best_cost)
            .choose(&mut rng)
            .copied()
            .unwrap();

        // Verificamos si ya hemos visto el gen actual en el pasado.
        // Si encontramos colisiones, entonces se trata de un camino sin salida
        if self.last_queens.contains(&self.queens) {
            // Forzamos algo de aleatoriedad
            self.queens[rng.gen_range(0..self.n)] = rng.gen_range(0..self.n);
        } else {
            self.last_queens.insert(self.queens.clone());
        }

        // Asignamos a la reina con mayor costo uno de los valores
        // que reducen más el costo
        self.queens[worst_pos] = new_cost;

        // Devolvemos el costo del tablero entero
        self.overall_cost()
    }

    /// Generamos un nuevo tablero de NxN para colocar N reinas.
    ///
    /// El código no esta pensado para tableros de tamaño menor a 4x4, por lo que
    /// si el tamaño deseado de tablero en `with_n` es menor a 4 no creamos la instancia.
    pub fn new(with_n: usize) -> Option<Self> {
        (with_n >= 4).then_some(NQueens {
            n: with_n,
            queens: vec![0; with_n],
            last_queens: HashSet::with_capacity(with_n * with_n),
            costs: vec![(0, 0, 0); with_n],
            verbose: false,
        })
    }
}

/// Implementación de la caracteristica Display
///
/// Esta característica se encarga de dictar cómo debe imprimirse
/// en consola una instancia de `NQueen`.
///
/// El valor mostrado depende de el campo `verbose`. Si es true se muestra
/// el costo de todas las reinas, de forma individual.
impl std::fmt::Display for NQueens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.n).try_for_each(|row| {
            (0..self.n)
                .map(|i| if self.queens[row] == i { '*' } else { '.' })
                .try_for_each(|val| write!(f, "{} ", val))?;

            if self.verbose {
                let cost = self.cost_of(row);
                let [cval, lval, rval] = { [cost[0], cost[1], cost[2]] };
                write!(
                    f,
                    " | ld:{lval:>2} rd:{rval:>2} cc:{cval:>2} | tt:{:>2}",
                    cost.iter().sum::<usize>()
                )?;
            }
            if row != self.n - 1 {
                writeln!(f)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
