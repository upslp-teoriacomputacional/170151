# Rust Program autómata finito determinista.
#### Institución: Universidad Politécnica de San Luis Potosí
#### Alumno: Yesenia America Morales Diaz de Leon
#### Matricula: 170151
#### Carrera: Ingeniería en Tecnologías de la Información
#### Materia: Teoría Computacional
#### Profesor: Juan Carlos González Ibarra
#### Objetivo:  Validar una expresión aritmética. Programar un autómata capaz de decidir si es o no una expresión aritmética valida.
#### Lenguaje: Rust
#### Repositorio: https://github.com/upslp-teoriacomputacional/170151

Para este programa se desarrollo un automata finito para validar operaciones matematicas. Para realizar esto se  utiliza una biblioteca que nos ayuda a manejar expresiones regulares. Al momento de utilizar esta libreria nos pide realizar el programa como un proyecto por lo que el codigo fuente se ubica en afd_170151/src/main.rs.

Se utilizan expresiones de:

Operador = +,-,*,/

Numeros = 0-9

El programa te pide ingresar una cadena para evaluar, la valida con la tabla de estados, te muestra la tabla de estados correspondiente a la cadena ingresada y te dice si es valida o no.

## Como se soluciono el problema 
Para manejar expresiones regulares en rust se utilizo la biblioteca Regex, que es una biblioteca de Rust para analizar, compilar y ejecutar expresiones regulares. Su sintaxis es similar a las expresiones regulares de estilo Perl, pero carece de algunas características como mirar alrededor y referencias inversas. A cambio, todas las búsquedas se ejecutan en tiempo lineal con respecto al tamaño de la expresión regular y el texto de búsqueda.
Se representa como una secuencia de instrucciones de código de bytes (dinámica) o como una función especializada de Rust (nativa). Se puede utilizar para buscar, dividir o reemplazar texto.
Otras librerias que se ocuparon fueron la de procesos y para ingresar datos por teclado.

Para utilizar la libreria Regex necesitamos mencionarla en nuestro Cargo.toml.
```
[package]
name = "afd_170151"
version = "0.1.0"
authors = ["Yesenia Morales <170151@upslp.edu.mx>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
regex = "1.3.9"
```
Y despues en nuestra cabecera del programa colocamos las librerias de la siguiente manera:

```rust
extern crate regex; //Libreria que nos ayuda a analizar
//compilar y ejecutar expresiones regualres
use regex::Regex;  // Libreria de expresiones regulares
use std::process;  //Para procesos
use std::io;      //Para entrada de datos por teclado
```
Ahora, con esto podemos definir nuestro conjunto de digitos y operadores que seran validos.
```rust
let digito = Regex::new(r"[0-9]").unwrap(); //Definir digito
let operador = Regex::new(r"(\+|\-|\*|/)").unwrap(); //Definir operadores
```
Para poder validar nuestros caracteres ingresados, primero necesitamos hacer una conversion a String para que Rust pueda realizar la comparación.
```rust
//Conversion para comparar caracter
let mut c= String::from("");
c.push(character);
let carac: &str = &c[..];
```
Entonces ahora si podremos comparar si es digito o operador
```rust
//Comparamos si es digito o operador
if digito.is_match(carac){
    return 0;
}else{
    if operador.is_match(carac){
        return 1;
    }else{
        if carac==fin {
        return 2;
        }else{
            //si no es ni un digito ni un operador entonces es un caracter no validp
            println!("Error el caracter: {} no es valido", carac);
            process::exit(0x0100);
        }
    }
    
}
```
En nuestra funcion principal crearemos una tabla para la tabla de transiciones. La cual es:
<pre>
|Estado	| Digitos | Operadores | Fin de cadena |
|  q0   |   q1	  |  Error	| Error         |
|  q1	| Error	  |   q2	| Error         |
|  q2	|   qf	  |  Error	| Error         |
|  qf	| Error	  |  Error	| Aceptacion    |
</pre>

Entonces nuestra tabla de transiciones seria:
<pre>
| 1 |  E |  E |
| E |  2 |  E |
| 3 |  E |  E |
| E |  E |  A |
</pre>

Para crear esta tabla en Rust se han cambiado los E por 4 y las A por 5

```rust
//Esta es la tabla de transiciones del automata AFD creado
let tabla: Vec<Vec<i32>>;
tabla = vec![ //E =4   A=5
    vec![1,4,4],
    vec![4,2,4],
    vec![3,4,4],
    vec![4,4,5]
];
```
Para leer nuestra cadena desde teclado

```rust
let mut cadena = String::new();     //Declarar variable
io::stdin().read_line(&mut cadena); //Leer la cadena desde el teclado
```

Ahora con un ciclo for in vamos validando caracter por caracter para ir recorriendo el automata

```rust
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
    }
}
```

Al final si el valor obtenido en estado es una E imprimimos cadena no valida, si el estado no es 3 que es el de aceptacion imprimimos cadena no valida y si el estado es 3 es una cadena de aceptacion
Si ingresamos una cadena valida nuestro resultado seria el siguiente:
<pre>
+-------------------------------------+
|    Ingrese una cadena a evaluar:    |
+-------------------------------------+
1+5
+--------------+---------+-----------+---------------+
|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |
+--------------+---------+-----------+---------------+
|     0        |  1      |  Digito   |     1         |
+--------------+---------+-----------+---------------+
|     1        |  +      | Operador  |     2         |
+--------------+---------+-----------+---------------+
|     2        |  5      |  Digito   |     3         |
+--------------+---------+-----------+---------------+
|     3        |         |Fin Cadena |               |
+--------------+---------+-----------+---------------+
|                Cadena Valida                       |
+----------------------------------------------------+
</pre>
