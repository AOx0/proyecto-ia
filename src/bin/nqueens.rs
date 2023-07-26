#![allow(non_snake_case)]

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Universidad Panamericana
/// Facultad de Ingeniería
///
/// Proyecto Final:  Inteligencia Artificial.
/// N-Queens Puzzle
///
/// Mayo 31, 2023
/// Osornio López Daniel Alejandro (0244685@up.edu.mx)
/// Hernandez Toledo Daniel (0243179@up.edu.mx)
///
/// Este archivo contiene el código del programa principal, es decir del ejecutable,
/// así como la declaración de estructuras y métodos que hacen posible resolver el problema
/// usando el método de Iterative Fix, esto es, mejora iterativa.
///
/// Para ejecutar el código se necesita tener `cargo`, el manager de paquetes del lenguaje
/// de programación Rust instalado.
/// Una vez instalado, desde la terminal, a nivel del archivo Cargo.toml (raíz del proyecto)
/// ejecutar:
///
///     cargo run --release
///
use nqueens::NQueens;
use std::borrow::Cow;

fn pedir_valor<V, F: Fn(&str) -> Result<V, Cow<'static, str>>>(
    msg: &'static str,
    buff: &mut String,
    transformer: F,
) -> Result<V, Cow<'static, str>> {
    use std::io::Write;
    use std::io::{stdin, stdout};

    let mut out = stdout().lock();
    write!(out, "{msg}").unwrap();
    out.flush().unwrap();

    buff.clear();
    let input = match stdin().read_line(buff) {
        Ok(n) if n > 0 => buff,
        Ok(_) => {
            return Err("No input provided, read 0 bytes from stdin"
                .to_string()
                .into())
        }
        Err(err) => return Err(format!("Error while reading stdin: {err}").into()),
    };

    transformer(input.trim())
}

/// Función main que ejecuta la logica principal del progrma
fn main() -> Result<(), Cow<'static, str>> {
    #[cfg(feature = "dhat")]
    let _profile = dhat::Profiler::new_heap();

    let mut buff = String::new();
    let N = pedir_valor("Ingresa el valor de N: ", &mut buff, |inp| {
        let val = inp
            .parse::<usize>()
            .map_err(|_| "Valor de N inválido. Ingresa un valor de N válido.")?;

        (val >= 4)
            .then_some(val)
            .ok_or("No se permiten valores de N menores a 4".into())
    })?;

    let verbose = pedir_valor(
        "Deseas mostrar información para cada paso? [y/N]: ",
        &mut buff,
        |inp| {
            Ok((inp == "y" || inp == "Y")
                .then_some(true)
                .unwrap_or_else(|| {
                    println!("Valor inválido, se considerará como que no desea información.");
                    false
                }))
        },
    )?;

    let wants_init = pedir_valor(
        "Deseas ingresar un estado inicial para el problema? [y/N]: ",
        &mut buff,
        |inp| {
            Ok((inp == "y" || inp == "Y")
                .then_some(true)
                .unwrap_or_else(|| {
                    println!("Valor inválido, se considerará como que no desea un estado inicial.");
                    false
                }))
        },
    )?;

    // Si quiere indicar estado inicial, lo leemos y parseamos en un vector.
    // Este `statement` regresa un problema de NQueens, sea random si hubo algun
    // error o asi lo quizo por defecto, o el especificado si el usuario ingresó
    // los datos de forma correcta
    let initial = wants_init.then(|| {
        println!(
            r#"
    Ingresa los valores del estado separados por comas
    Un ejemplo de estado es [0, 3, 2, 1] para una N = 4
        En el ejemplo:
            - La reina 0 esta en la fila 0 y columna 0
            - La reina 1 esta en la fila 1 y columna 3
            - La reina 2 esta en la fila 2 y columna 2
            - La reina 3 esta en la fila 3 y columna 1
            - Todos los valores son menores a N
            - Los valores estan separados por ','
"#
        );

        pedir_valor("Ingresa ahora el estado: ", &mut buff, |inp| {
            let mut array = Vec::with_capacity(N);
            inp.trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .try_for_each(|val| {
                    let res = val.trim().parse::<usize>();

                    if let Ok(res) = res {
                        if res >= N {
                            Err(format!("Valor '{}' mayor o igual a N ({})", val, N))
                        } else {
                            array.push(res);
                            Ok(())
                        }
                    } else {
                        Err(format!("Valor '{}' no es un número válido", val))
                    }
                })?;
            if array.len() != N {
                Err("Not enough values. Fallbacking to random initial state".into())
            } else {
                Ok(array)
            }
        })
    });

    let nqueens = NQueens::new(N).unwrap().with_verbose(verbose);
    let mut nqueens = match initial {
        Some(Ok(a)) => nqueens.with_state(&a).unwrap(),
        Some(Err(err)) => {
            println!("{}", err);
            nqueens.into_random_state()
        }
        None => nqueens.into_random_state(),
    };

    let initial_state = nqueens.clone();
    verbose.then(|| println!("{}\n", nqueens));

    // Resolvemos el problema
    let mut iterations = 0;
    while nqueens.step() != 0 {
        verbose.then(|| println!("{}\n", nqueens));
        iterations += 1;
    }

    // Mostramos el resultado
    println!("Terminado con {iterations} iteraciones.");
    println!("Estado inicial: ");
    println!("{initial_state}\n");
    println!("Estado final: ");
    println!("{nqueens}\n");
    Ok(())
}
