/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra
 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 06/10/2020
 Objetivo:  Validar una expresión aritmética. Programar un autómata 
 capaz de decidir si es o no una expresión aritmética valida.
*/

extern crate regex; //Libreria que nos ayuda a analizar
//compilar y ejecutar expresiones regualres
use regex::Regex;  // Libreria de expresiones regulares
use std::process;  //Para procesos
use std::io;      //Para entrada de datos por teclado

//Definimos la funcion caracter
fn caracter(character: char) -> i32 {
    let mut fin=""; //Declaracion variables
    let digito = Regex::new(r"[0-9]").unwrap(); //Definir digito
    let operador = Regex::new(r"(\+|\-|\*|/)").unwrap(); //Definir operadores
    //Conversion para comparar caracter
    let mut c= String::from("");
	c.push(character);
	let carac: &str = &c[..];

    //Comparamos si es digito o operador
    if digito.is_match(carac){
        return 0;
    }else{
        if operador.is_match(carac){
            return 1;
        }else{
            if carac==fin {
                //Llego al fin de la cadena
                process::exit(0x0100);
            }else{
                //si no es ni un digito ni un operador entonces es un caracter no validp
               //Regresa un 2
                return 2;
            }
        }
        
    }
}

//Definimos la funcion encabezado
fn encabezado(){
    println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
}

//Definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadosig: i32, carac: char, simbolo: &str, estado: i32){
    println!("|     {}        |  {}      | {} |     {}         |", estadosig, carac, simbolo,estado);
    body();
}

//Solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body(){
    println!("+--------------+---------+-----------+---------------+");
}

fn main(){
    //Esta es la tabla de transiciones del automata AFD creado
    let tabla: Vec<Vec<i32>>;
    tabla = vec![ //E =4   A=5
        vec![1,4,4],
        vec![4,2,4],
        vec![3,4,4],
        vec![4,4,5]
    ];
    //tabla=[[1,"E","E"],["E",2,"E"],[3,"E","E"],["E","E","A"]]
    let mut estado: i32 = 0; //Declaramos variables de estaod y simbolo
    let mut simbolo: String = "".to_string();
    let mut fin="";
    println!("+-------------------------------------+");
    println!("|    Ingrese una cadena a evaluar:    |");
    println!("+-------------------------------------+");
    let mut cadena = String::new();     //Declarar variable
    io::stdin().read_line(&mut cadena); //Leer la cadena desde el teclado
    body();
    encabezado();
    //Ciclo para recorrer la cadena
    for  character in cadena.chars(){
        let mut estadosig: i32=estado;
        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter= caracter(character);
        //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado = tabla[estado as usize][charcaracter as usize];
        //Vemos que nos regresa caracter y asignamos si es digito o operador
        if charcaracter == 0 {
            simbolo = " Digito  ".to_string();
        }else if charcaracter == 1 {
            simbolo = "Operador ".to_string();
        }else if charcaracter== 2 {
            //al concluir si el estado no es 3 que es el de aceptacion imprimimos cadena no valida    
            if estado!=3 {
                println!("|              Cadena No Valida :(                   |");
                println!("+----------------------------------------------------+");
            }
            println!("Error el caracter: {} no es valido", character);
            process::exit(0x0100);
            
        }
        //Si el valor obtenido es una E imprimimos cadena no valida
        if estado==4 {
            println!("|     {}      |  {}      | {} |     {}         |", estadosig, character, simbolo,estado);
            body();
            println!("|              Cadena No Valida :(                   |");
            println!("+----------------------------------------------------+"); 
            process::exit(0x0100);
        }
        contenido(estadosig,character,&simbolo,estado);
        //si el estado es 3 es una cadena de aceptacion
        if estado==3 {
            println!("|     {}        |         |Fin Cadena |               |", estado);
            body();
            println!("|                Cadena Valida                       |");
            println!("+----------------------------------------------------+");
            process::exit(0x0100);
        }
    }
} 