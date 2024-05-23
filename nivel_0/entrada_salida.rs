/////////////////////////////////////
//Pedro Velasco Santana            //
//                                 //
//Archivo: entrada_salida.rs       //
/////////////////////////////////////

//Programa simple para mostrar la interacción con el usuario por consola

use std::io; //Librería para E/S

fn main(){
    println!("Ingresa tu nombre: \n");
    let mut nombre = String::new(); //mut se refiere a que se puede cambiar el valor de la variable xd

    io::stdin().read_line(&mut nombre).expect("[-] Fallo al leer la entrada");

    println!("\nHola, {}", nombre);

}