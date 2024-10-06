use std::io;

fn main() {
    println!("Calculador en rust");

    loop {
        let num1 = obtener_numero("Ingresa el numerito gato: ");
        let operacion = obtener_operacion("Ingresa la opereishon: ");
        let num2 = obtener_numero("Ingresa el otro numero gil: ");

        let resultado = match operacion.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("No se puede divir por cero bobo");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("No es valido ese operador salamin");
                continue;
            }
        };
        println!(
            "El resultado de {} {} {} es: {}",
            num1, operacion, num2, resultado
        );

        let continuar = obtener_input("Quere' seguir mono? (s/n): ");
        if continuar.trim() != "s" {
            println!("Chau ñeri!");
            break;
        }
    }
}

fn obtener_numero(mensaje: &str) -> f64 {
    loop {
        println!("{}", mensaje);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error perro, no pude leer la linea.");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Mono, ese numero no vale pa' e'to, mete otro: "),
        }
    }
}

fn obtener_operacion(mensaje: &str) -> String {
    println!("{}", mensaje);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error perro, no pude leer la linea.");

    input.trim().to_string()
}

fn obtener_input(mensaje: &str) -> String {
    println!("{}", mensaje);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error perro, no pude leer la linea.");

    input.trim().to_string()
}
