# Rust Program autómata finito determinista.
#### Institución: Universidad Politécnica de San Luis Potosí
#### Alumno: Yesenia America Morales Diaz de Leon
#### Matricula: 170151
#### Carrera: Ingeniería en Tecnologías de la Información
#### Materia: Teoría Computacional
#### Profesor: Juan Carlos González Ibarra
#### Objetivo:  Validar una expresión regular
#### Lenguaje: Rust
#### Repositorio: https://github.com/upslp-teoriacomputacional/170151

Para este programa se desarrollo un programa que valide expresiones reguales. Para realizar esto se  utiliza una biblioteca que nos ayuda a manejar expresiones regulares. Al momento de utilizar esta libreria nos pide realizar el programa como un proyecto por lo que el codigo fuente se ubica en re_170151/src/main.rs.

Se utilizan expresiones de:

Operador = +,-,*,/,=,%

Numeros = 0-9

Alfabeto = a-zA-Z
 
El programate evalua una expresion dada, donde nos dice de que esta compuesta esa expresion.

## Como se soluciono el problema 
Para poder realizar el programa se utilizo la libreria de Regex, la cual nos ayuda a validar expresiones regulares, ademas utilizamos HashMap, el tipo HashMap< K,V > almacena un mapeo de claves de tipo K a valores de tipo V. Esto lo hace a través de una función de hashing, que determina cómo coloca estas claves y valores en la memoria, debemos realizar una llamada a la librería collections para poder trabajar con HashMap. 

Para utilizar la libreria Regex necesitamos mencionarla en nuestro Cargo.toml.
```
[package]
name = "re_170151"
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
//compilar y ejecutar expresiones regulares
use regex::Regex;  // Libreria de expresiones regulares
use std::collections::HashMap; //Libreria para utilizar HashMap
```
Ahora, con esto podemos definir nuestro conjunto que seran validos.
```rust
let identifier = Regex::new(r"[[:alpha:]]").unwrap(); //Definir alfabeto
let integer = Regex::new(r"[0-9]|[0-9];").unwrap(); //Definir digito
let operator = Regex::new(r"(\+|\-|\*|%|/|=)").unwrap(); //Definir operadores
```
Para poder validar nuestra expresion, primero necesitamos separarla, mediante el metodo split para validar a que corresponde cada parte.
```rust
let source_code = "int num = 25;".split(" "); //Separar la expresion en palabras por espacio
```
Ahora, para guardar nuestro conjunto de a que corresponde y su valor creamos un HashMap
```rust
let mut tokens = HashMap::new();
```

Ahora vamos validando palabra por palabra de la expresion dada si corresponde a un tipo de dato, un indentificador, un operador o un entero asi como el final de expresion que debe ser punto y coma (;).
```rust
//Vamos comparando palabra por palabra de la expresion
for word in source_code{ 
    if word == "str" || word == "int" || word == "bool" { //Comparamos si es un tipo de dato
        tokens.insert("DATATYPE".to_string(), word.to_string());
    }
    if identifier.is_match(word){ //Comparamos si es un identificador
        tokens.insert("IDENTIFIER".to_string(), word.to_string());
    }
    if operator.is_match(word){ //Comparamos si es un operador
        tokens.insert("OPERATOR".to_string(), word.to_string());
    }
    if integer.is_match(word){   //Comparamos si es un entero
        let len=word.len();  
        if word[len-1..len].to_string() == ";"{ //Si tiene el punto y coma final
            tokens.insert("INTEGER".to_string(), word[0..(len-1)].to_string());
            tokens.insert("END_STATEMENT".to_string(), ';'.to_string());
        }else{ //Si no contiene punto y coma final.
            tokens.insert("INTEGER".to_string(), word.to_string());
        }
    }
}
```

La expresion regular de ejemplo dada al programa fue: ``` "int num = 25;" ```

Y al ejecutar el ´programa nuestra salida sera la siguiente:
<pre>
OPERATOR: "="
IDENTIFIER: "num"
END_STATEMENT: ";"
DATATYPE: "int"
INTEGER: "25"
</pre>
