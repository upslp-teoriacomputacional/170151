/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra
 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 23/10/2020
 Objetivo:  Validar expresiones regulares
*/

extern crate regex; //Libreria que nos ayuda a analizar
//compilar y ejecutar expresiones regulares
use regex::Regex;  // Libreria de expresiones regulares
use std::collections::HashMap; //Libreria para utilizar HashMap


fn main(){
let mut tokens = HashMap::new();
let identifier = Regex::new(r"[[:alpha:]]").unwrap(); //Definir alfabeto
let integer = Regex::new(r"[0-9]|[0-9];").unwrap(); //Definir digito
let operator = Regex::new(r"(\+|\-|\*|%|/|=)").unwrap(); //Definir operadores
let source_code = "int num = 25;".split(" "); //Separar la expresion en palabras por espacio
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
for (tipo, car) in &tokens { //Imprimimos nuestro HashMap
    println!("{}: \"{}\"", tipo, car);
}
}

fn variablePROLOG(w: String)->bool{
    let mut w1: Vec<char> = Vec::new();
    for chars in w.chars()
    {
        w1.push(chars);
    }
    //(str)-->bool. True si "w" es un nombre de variable correcto
    //El primer caracter es una mayuscula o un subrayado
    let w2:Vec<char>=w1[0].to_uppercase().collect();
    if w1[0].is_alphabetic() && w1[0] == w2[0] || w1[0] == '_'
    {
        w1.reverse();
        w1.pop();
        w1.reverse(); // Se quita el primer caracter
        //Mientras queden caracteres en "w" y el primer caracter actual sea un alfanumerico o un subrayado, todo esta bien
        while  !w1.is_empty() && (w1[0].is_numeric() || w1[0] == '_') 
        {
            w1.reverse();
            w1.pop();
            w1.reverse(); // Se quita el primer caracter
            if w1.is_empty() {
                //Si ya no quedan elementos a revisar, es una variable PROLOG
                return true;
            }
        }
    }
    return false;
}
