/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra
 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 05/11/2020
 Objetivo:  Validar un palindromo
*/

use std::io;      //Para entrada de datos por teclado


fn main(){
    println!("Palindromo:");
    let mut cadena = String::new();     //Declarar variable cadena
    io::stdin().read_line(&mut cadena); //Leer la cadena desde el teclado
    let mut num = cadena.len();         //Obtiene tamaño de la cadena
    let mut tamPila = (num/2);          //Calcula el tamaño de la pila
    let mut w1: Vec<char> = Vec::new(); //Crea la pila
    let cadenaP: &str = &cadena[..tamPila]; //Obtiene solamente la cadena que se guardara en la pila
    println!("\nPila:");
    for chars in cadenaP.chars() //Guarda los datos en la pila
    {
        w1.push(chars);
        print!("{}",chars);
    }
    println!("\n\nDesapilar:"); 
    w1.pop(); //Desapila el valor de enmedio 
    while let Some(top) = w1.pop() { //Desapila los restantes valores 
        print!("{}", top);
    }

}
