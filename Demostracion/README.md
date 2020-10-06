# Programa de Rust compuertas logicas
#### Institución: Universidad Politécnica de San Luis Potosí
#### Alumno: Yesenia America Morales Diaz de Leon
#### Matricula: 170151
#### Carrera: Ingeniería en Tecnologías de la Información
#### Materia: Teoría Computacional
#### Profesor: Juan Carlos González Ibarra
#### Objetivo: Programa que muestre las tablas de verdad de los operadores lógicos básicos or, and, not y xor.
#### Lenguaje: Rust
#### Repositorio: https://github.com/upslp-teoriacomputacional/170151

## Como se soluciono el problema 
Las expresiones de los operadores binarios están todas escritas con notación infija. Los operadores logicos basicos O, Y, NO y O Exclusivo se representan en rsut como || (or), && (and), ! (not) y ^(xor). Los operadores || y && pueden aplicarse a operandos de tipo booleano. Se diferencian de | y & en que el operando de la derecha solo se evalúa cuando el operando de la izquierda no determina ya el resultado de la expresión. Es decir, ||solo evalúa su operando de la derecha cuando el operando de la izquierda evalúa a false, y &&solo cuando evalúa a true.
Se realizara una comparacion entre dos boleanos para crear las tablas de verdad de estos operadores logicos.
Al utilizar || y && las variables deben accederse por referencia.

Se crea una lista con los dos valores booleanos: TRUE y FALSE
```rust
    let booleanos : [bool;2] = [false, true];
```

Mediante un ciclo for anidado vamos comparando las dos variables entre si, en sus dos valores: verdadero y falso.
Es decir:
x en verdadero se compara con y en verdadero y falso.
x en falso se compara con y en verdadero y falso.

Para realizar el OR utilizamos la expresion
```rust
    x || y
```

Entonces para realizar la tabla de verdad de OR lo hacemos de la siguiente manera:

```rust
for x in booleanos.iter(){
    for y in booleanos.iter(){
        println!("{}\t{}\t{}",x,y, *x || *y ); // x OR y = x|y
    }
}
```
Para realizar el AND utilizamos la expresion
```rust
    x && y
```

Entonces para realizar la tabla de verdad de OR lo hacemos de la siguiente manera:

```rust
for x in booleanos.iter(){
    for y in booleanos.iter(){
        println!("{}\t{}\t{}",x,y, *x && *y ); // x AND y = x&y
    }
}
```
Para realizar el NOT utilizamos la expresion
```rust
    !x
```
Entonces para realizar la tabla de verdad de NOT lo hacemos solamente con el valor de una variable de la siguiente manera:

```rust
for x in booleanos.iter(){
    println!("{}\t{}",x,!x);  // not x = !x
}
```

Para realizar el XOR utilizamos la expresion
```rust
    x ^ y
```

Entonces para realizar la tabla de verdad de XOR lo hacemos de la siguiente manera:

```rust
for x in booleanos.iter(){
    for y in booleanos.iter(){
        println!("{}\t{}\t{}",x,y, x ^ y ); // x ^ y 
    }
}
```