/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra
 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 16/10/2020
 Objetivo:  Validar una cadena. Programar un autómata finito 
 no deterministico capaz de decidir si es o no una cadena
 valida para la expresion regular a evaluar que es a*ba*.
*/

extern crate regex; //Libreria que nos ayuda a analizar
//compilar y ejecutar expresiones regualres
use regex::Regex;  // Libreria de expresiones regulares
use std::process;  //Para procesos
use std::io;      //Para entrada de datos por teclado


//Definimos la funcion caracter
fn caracter(character: char)-> i32{
    let mut fin="\r"; //Declaracion variables
    let a = Regex::new(r"a").unwrap();
    let b = Regex::new(r"b").unwrap();
    //Conversion para comparar caracter
    let mut c= String::from("");
	c.push(character);
	let carac: &str = &c[..];
   //Si no comparamos si la entrada es a o b, si ya llego a su fin la cadena o si ingresa algo diferente.
    if a.is_match(carac)
    {
        return 1;       //a
    }
    else if b.is_match(carac) 
    {
        return 2;       //b
    }
    else if carac==fin
    {
        return 0;       //Fin de cadena, ya esta vacia
    }else{
        return 3;       //Caracter no valido
    }  
}

//Definimos al la funcion  encabezado
fn encabezado()
{
    println!("| Edo. Actual  |Caracter | Simbolo   |Edo. Siguiente |");
    body();
}

//Definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadosig: i32, carac: char, simbolo: &str, estado: i32){
    println!("|     {}        |   {}     |   {}   |     {}         |", estadosig, carac, simbolo, estado);
    body();
}
//Solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body()
{
    println!("+--------------+---------+-----------+---------------+");
}
   
fn main(){
    //Establecer la tabla de transiciones del automata AFN 
    //Como no acepta el 10 y 11 se utilizan las letras D y O.
    //Que en codigo ascci serian 68 y 79.
    let tabla: Vec<Vec<char>>;
    tabla = vec![ 
                // ε   A   B   Fin
            vec! ['1','E','E','E'],     
            vec! ['2','E','E','E'],     
            vec! ['E','3','E','E'],     
            vec! ['4','E','E','E'],     
            vec! ['5','E','E','E'],     
            vec! ['E','E','6','E'],    
            vec! ['7','E','E','E'],     
            vec! ['8','E','E','E'],     
            vec! ['E','9','E','E'],     
            vec! ['D','E','E','E'],     
            vec! ['O','E','E','E'],     
            vec! ['E','E','E','A']      
            //10='D' 68, 11='O' 79
    ];
    let mut estado: i32  = 0; //Declaramos variables de estado
    let mut simbolo: String = "".to_string();
    //Leemos la cadena por teclado
    println!("+-------------------------------------+");
    println!("|    Ingrese una cadena a evaluar:    |");
    println!("+-------------------------------------+");
    let mut cadena = String::new();     //Declarar variable cadena
    io::stdin().read_line(&mut cadena); //Leer la cadena desde el teclado
    //Imprimir tabla
    body();
    encabezado();
    //Ciclo para recorrer la cadena de caracter por caracter
    for  character in cadena.chars(){
        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut newcaracter= caracter(character);
        //Si nuestro estado es igual a un estado con trasicion vacia nos metemos a un ciclo
        while estado == 0 || estado == 1 || estado == 3 || estado == 4 || estado == 6 || estado == 7 || estado == 9 || estado == 10 
        {
            let mut estadosig: i32=estado;
            //Verificamos si hay que tomar la segunda salida de los estados que tienen dos salidas.
            //Si el caracter es b y el estado es 1 se va hacia su segunda salida que es 4
            if character == 'b' && estado ==1 
            {
                estado=4;
            } //Si el caracter es a y su estado esta en 4 regresamos al estado 1
            else if character == 'a' && estado ==4
            {
                estado=1;
            } //Si el caracter es a y el estado 10, regresamos al estado 7
            else if character == 'a' && estado == 10
            {
                estado=7;
            } //Si el caracter esta en vacio despues de b, se dirige a el estado 10
            else if character == '\r' && estado == 7
            {
                estado=10;
            }else 
            {    //Si no se cumple ninguna de esas caracteristicas avanza de estado.
                estadosig=estado; 
                newcaracter = 0; // La columna que debe tomar en la tabla es la 0 por que tenemos una transicion vacia
                if estado==9
                { // Si el estado es igual a 9, asignamos a estado 10, debido al arreglo que tenemos en la tabla
                    estado=10;
                }else if estado==10
                { //Y hacemos lo mismo con el estado 10
                    estado=11;
                }else{ //Si no pues avanzamos de estado normal, tomando los valores de nuestra tabla
                    estado=(tabla[ estado as usize][newcaracter as usize])as i32 -48 ;
                }
            }
            contenido(estadosig,' ',"     ",estado);
           
        } 
        if estado==11 { //Si hemos llegado a nuestro estado 11 
            if character == '\r' //Si la cadena ya esta vacia es valida
            {
                println!("|     {}       |         | Fin Cadena |              |", estado);
                body();
                println!("|                    Cadena Valida                   |");
            }else{ //Si sigue teniendo caracteres es no valida
                println!("|     {}       |   {}     |           |    {}         |", estado, character,estado);
                body();
                println!("|                    Cadena No Valida                |");
            }      
            println!("+----------------------------------------------------+");
            process::exit(0x0100);
        }
        //Si no seguimos nuestro proceso
        //Asignamos estado
        let mut estadosig: i32=estado; 
        if estado == 5 && character == '\r' 
        {
            println!("|     {}        |          |          |    {}        |", estado,estado);
            body();
            println!("|                    Cadena No Valida                |");
            println!("+----------------------------------------------------+");
            process::exit(0x0100);
       }
       if estado == 8 && character == 'b'
        {
            println!("|     {}        |   {}     |   a     |    {}        |", estadosig, character,estado);
            body();
            println!("|                    Cadena No Valida                |");
            println!("+----------------------------------------------------+");
            process::exit(0x0100);
       }
       
      //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut newcaracter= caracter(character);
        //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado=(tabla[ estado as usize][newcaracter as usize])as i32 -48 ;
        if newcaracter == 1
        {
            simbolo = "  a  ".to_string();
        }else if newcaracter == 2
        {
            simbolo = "  b  ".to_string();
        }else if newcaracter == 3
        {
            simbolo = "     ".to_string();
            //si no es ni una a ni una b entonces es un caracter no valido
            println!("|     {}        |   {}     |   {}    |    {}        |", estadosig, character, simbolo,estadosig);
            body();
            println!("|                    Cadena No Valida                |");
            println!("+----------------------------------------------------+");
            println!("Error el caracter: {} no es valido",character);
            process::exit(0x0100); 
        }
        
        contenido(estadosig,character,&simbolo,estado);
    }
   
     
} 