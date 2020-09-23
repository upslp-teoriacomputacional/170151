/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra

 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 21/09/2020

 Objetivo: Programa que muestre las tablas de verdad de los operadores lógicos básicos or, and, not y xor.
*/
fn main(){
    //Comenzamos creando una lista con los dos valores booleanos: TRUE y FALSE
    let booleanos : [bool;2] = [false, true];
    
    //Tabla de verdad de OR
    //Impresion de cabecera de la tabla
    println!("x\ty\tx or y");
    println!("-----------------------");
    //Realizamos un ciclo for anidado para realizar las comparaciones entre ambos valores.
    //Vamos comparando las dos variables entre si, en sus dos valores: verdadero y falso. Es decir:
    //x en verdadero se compara con y en verdadero y falso.
    //x en falso se compara con y en verdadero y falso.
    for x in booleanos.iter(){
        for y in booleanos.iter(){
            println!("{}\t{}\t{}",x,y, *x || *y ); // x OR y = x|y
        }
    }
    println!("");
    
    //Tabla de verdad de and
    //Impresion de cabecera de la tabla
    println!("x\ty\tx and y");
    println!("-----------------------");
    //Realizamos un ciclo for anidado para realizar las comparaciones entre ambos valores.
    //Vamos comparando las dos variables entre si, en sus dos valores: verdadero y falso. Es decir:
    //x en verdadero se compara con y en verdadero y falso.
    //x en falso se compara con y en verdadero y falso.
    for x in booleanos.iter(){
        for y in booleanos.iter(){
            println!("{}\t{}\t{}",x,y, *x && *y ); // x AND y = x&y
        }
    }
    println!("");
    
    //Tabla de verdad de not
    //Impresion de cabecera de la tabla
    println!("x\tnot x");
    println!("--------------");
    //Realizamos un ciclo for para hacer la negacion de los dos valores de la variable
    for x in booleanos.iter(){
        println!("{}\t{}",x,!x);  // not x = !x
    }
    println!("");
    
    //Tabla de verdad de ^
    //Impresion de cabecera de la tabla
    println!("x\ty\tx^y");
    println!("-----------------------");
    //Realizamos un ciclo for anidado para realizar las comparaciones entre ambos valores.
    //Vamos comparando las dos variables entre si, en sus dos valores: verdadero y falso. Es decir:
    //x en verdadero se compara con y en verdadero y falso.
    //x en falso se compara con y en verdadero y falso.
    for x in booleanos.iter(){
        for y in booleanos.iter(){
            println!("{}\t{}\t{}",x,y, x ^ y ); // x ^ y 
        }
    }
    println!("");
}