# Rust Program to Illustrate Different Set Operations
#### Institución: Universidad Politécnica de San Luis Potosí
#### Alumno: Yesenia America Morales Diaz de Leon
#### Matricula: 170151
#### Carrera: Ingeniería en Tecnologías de la Información
#### Materia: Teoría Computacional
#### Profesor: Juan Carlos González Ibarra
#### Objetivo: Programa para realizar diferentes operaciones de conjuntos. Funciones que sirven para realizar las operaciones y como se manejan los conjuntos.
#### Lenguaje: Rust
#### Repositorio: https://github.com/upslp-teoriacomputacional/170151

## Como se soluciono el problema 
Para manejar los conjuntos en rust se utilizó el módulo std::collections::HashSet.
La biblioteca de colección estándar de Rust nos proporciona implementaciones eficientes de las estructuras de datos de programación de propósito general. La colección para conjuntos es HashSet, con este podemos crear conjuntos, imprimirlos y realizar operaciones entre ellos como la unión, intersección, diferencia, entre otros.

Podemos Crear un conjunto con valores o un conjunto vació de la siguiente manera:
```
let A : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
let B : HashSet<i32> = HashSet::new();
```
y podemos lograr su impresión de la siguiente manera: 
```println!("{:?}", &A);
```
Entre las cosas básicas que podemos realizar con un conjunto encontramos el transformar un vector que ya existía en conjunto:
```let A = [1, 2, 3]; 
let conjuntoA: HashSet<_> = A.iter().collect(); 
```
así como saber si un conjunto contiene un elemento en especifico, donde nos regresa un verdadero o un falso:
```let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
A.contains(&1));
```
Podemos agregar elementos a un conjunto, ya se vacío o que ya contenga elementos. Así como podemos copiar los elementos de un conjunto a otro.
```let A : HashSet<_> = [0, 1, 2, 3, 4, 5].iter().collect();
let B: HashSet<_> = A.clone();
B.insert(&9);
```
También se pueden quitar elementos de nuestro conjunto, pero también podemos limpiarlo completamente, de la siguiente manera:
```let mut A : HashSet<_> = [0, 1, 2, 3, 4, 5].iter().collect();
A.remove(&2);
A.clear();
```
Ahora, debemos tener en cuenta que en estos ejemplos se han utilizado enteros, pero ¿Qué pasa con las cadenas? Los strings en Rust no cuentan con un método de iteración en su estructura por lo que al querer convertir una cadena en conjunto hay que utilizar el método de inserción de letra por letra, como se muestra a continuación
```let C = "Hola mundo";
let mut conjuntoC: HashSet<char> = HashSet::new();
for caracter in C.chars(){
    conjuntoC.insert(caracter);
}
```

## Comandos para ejecutar operaciones
Para saber como funcionan las operaciones de conjuntos, declararemos dos conjuntos iniciales, recordando que estos comandos nos regresan el conjunto correspondiente.:
```let A : HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
let B : HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
```
**Unión.** . - La unión es una operación que resulta en otro conjunto, cuyos elementos son los mismos de los conjuntos iniciales. Por ejemplo: AUB, donde el resultado sería los elementos de ambos conjuntos.
```A.union(&B)
```
**Intersección**. - Se da otro conjunto en el que se encuentran los elementos que pertenecerá a A y que también pertenecerán a B es decir los elementos comunes en ambos conjuntos. A∩B
```A.intersection(&B)
```
**Diferencia**. - La diferencia de A – B es un conjunto al que pertenece todos los elementos de A que no pertenecen a B.
```A.difference(&B)
```
**Diferencia simétrica**. - es una operación cuyo resultado es otro conjunto que contiene a aquellos elementos que pertenecen a ambos conjuntos pero que no comparten, es decir, aquellos elementos que no son los mismos.
```A.symmetric_difference(&B) // A^B
B.symmetric_difference(&A) // B^A
```
Para los siguientes ejemplos declararemos dos nuevos conjuntos, recordando que los siguientes comandos solamente nos regresan un True o False:
```let B: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect(); 
let A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect(); 
```
**Subconjunto**.- Es un conjunto de elementos que tienen las mismas características y que está incluido dentro de otro conjunto más amplio.
```A.is_subset(&B)
```
**Superconjunto**. - Es un conjunto de elementos donde los elementos de un subconjunto existen en él.
```B.is_superset(&A)
```
