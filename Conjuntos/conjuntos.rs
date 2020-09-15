/*
 Universidad Politécnica de San Luis Potosí
 Ingeniería en Tecnologías de la Información
 Yesenia América Morales Díaz de León
 Matricula: 170151
 Materia: Teoría Computacional
 Profesor: Juan Carlos Gónzalez Ibarra

 Repositorio: https://github.com/upslp-teoriacomputacional/170151
 Escrito: 14/09/2020

 Objetivo: Program to perform different set operations like in mathematics
*/

use std::collections::HashSet;

fn main(){
    //Definicion de tres conjuntos
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    let C : HashSet<i32> = HashSet::new();

    //Impresion de conjuntos
    println!("{:?}", &A);
    println!("{:?}", &B);
    println!("{:?}", &C);
    //Set Section
    pertenencia();
    transformar();
    quitar();
    limpiarConjunto();
    copiar();
    agregar();
    // Set Operations
    union();
    interseccion();
    diferencia();
    simetrica();
    subconjunto();
    superconjunto();
}
/* Set Section */
fn pertenencia(){
    //Declaracion de conjunto
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    println!(" 1 pertenece al conjunto A: {}",A.contains(&1));   //1 in A
    println!(" 1 no pertenece al conjunto A: {}",!A.contains(&1));  //1 not in A 
    println!(" 10 pertenece al conjunto A:{}",A.contains(&10));  //10 in A 
    println!(" 10 no pertenece al conjunto A:{}",!A.contains(&10)); //10 not in A 
}

fn transformar(){
    let A = [1, 2, 3]; //Se declaran un arreglo
    let conjuntoA: HashSet<_> = A.iter().collect(); // Transforma conjunto A con .iter().collect()
    println!("The set is A: {:?}", &conjuntoA); // Set A
    let B = vec![1, 2, 3, 4, 5]; //Se declara un vector
    let conjuntoB: HashSet<_> = B.iter().collect(); // Transforma conjunto B con .iter().collect()
    println!("The set is B: {:?}", &conjuntoB); // Set B
    let C = "Hola mundo"; //Se declara una cadena
    // Con un for each para recorrer la cadena caracter
    // por caracter para agregarlo al conjunto
    let mut conjuntoC: HashSet<char> = HashSet::new();
    for caracter in C.chars(){
        conjuntoC.insert(caracter);
    }
    println!("The set is C: {:?}", &conjuntoC); // Set C
}

fn quitar(){
    // Declaracion de conjunto
    let mut A : HashSet<_> = [0, 1, 2, 3, 4, 5].iter().collect();
    // Remover el elemento 2 del conjunto
    A.remove(&2);
    println!("The set after to delete: {:?}", &A); // Set A
}

fn limpiarConjunto(){
    // Declaracion de conjunto
    let mut A : HashSet<_> = [0, 1, 2, 3, 4, 5].iter().collect();
    // Remover el elemento 2 del conjunto
    A.clear();
    println!("The set clear: {:?}", &A); // Set A
}

fn copiar()
{
    // Declaracion de conjunto
    let A : HashSet<_> = [0, 1, 2, 3, 4, 5].iter().collect();
    // Copiar conjunto A en conjunto B
    let B: HashSet<_> = A.clone();
    // Impresion de conjuntos
    println!("Set A = {:?} compare set B = {:?}", &A,&B)
}

fn agregar(){
    // Declaracion de conjunto vacio
    let mut B = HashSet::new();
    B.insert(&987); // Agregar elemento al conjunto
    println!("The new set B= {:?}", &B); // Imprimir el conjunto B
}
/* Set Operations */
fn union(){
    // Definicion de dos conjuntos
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    // Impresion de la union de los conjuntos
    println!("The union = {:?}", A.union(&B)); 
}

fn interseccion(){
    // Definicion de dos conjuntos
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    // Impresion de la interseccion de los dos conjuntos
    println!("The intersection = {:?}", A.intersection(&B)); 
}

fn diferencia(){
    // Definicion de dos conjuntos
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    // Impresion de la diferencia de conjuntos A-B
    println!("The diference = {:?}", A.difference(&B)); 
}

fn simetrica(){
    // Definicion de tres conjuntos
    let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    let C : HashSet<_> = [].iter().collect();
    // Impresion de las diferencia simetrica entre conjuntos
    println!("The symmetric_difference = {:?}",A.symmetric_difference(&B)); // A^B
    println!("The symmetric_difference = {:?}",B.symmetric_difference(&A)); // B^A
    println!("The symmetric_difference = {:?}",A.symmetric_difference(&C)); // A^C
    println!("The symmetric_difference = {:?}",B.symmetric_difference(&C)); // B^C
}

fn subconjunto(){
    // Definicion de dos conjuntos
    let B: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect(); 
    let A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect(); 
    // Imprime si A es subconjunto de B
    println!("The subset = {:?}",A.is_subset(&B));
    // Imprime si B es subconjunto de A
    println!("The subset = {:?}",B.is_subset(&A));
}

fn superconjunto(){
    // Definicion de dos conjuntos
    let B: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect(); 
    let A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect(); 
    // Imprime si B es superconjunto de A
    println!("The superset = {:?}",B.is_superset(&A));
    // Imprime si A es suerconjunto de B
    println!("The superset = {:?}",A.is_superset(&B));
}

/*
    Rust tiene arreglos. Los arreglos vacios se ven de la siguiente forma [].
    El siguiente es un arreglo de un elemento ["a"] y tambien [3]. Aqui hay uno 
    con tres elementos ["ball", 3.14, -2].
*/