mod my_lib {

    // Приватный трейт – он не экспортируется.
    // Его можно реализовать только внутри этого модуля.
    trait Sealed {
    }

    // Публичный трейт, который требует Sealed как супертрейт.
    // Это означает: чтобы реализовать `PublicTrait`, тип обязан 
    //  реализовать `Sealed`.    
    pub trait PublicTrate: Sealed {
        fn public_method(&self) ;
    }

    // Реализуем Sealed для типов, которые мы хотим допустить.    
    impl Sealed for i32 {
    }

    impl Sealed for String {
    }

    // Реализуем PublicTrait для этих типов.
    impl PublicTrate for i32 {
        fn public_method(&self) {
            println!("i32: {}", self) ;
        }
    }

    impl PublicTrate for String {
        fn public_method(&self) {
            println!("String: {}", self) ;
        }
    }
}

use std::fmt::Display;

use my_lib::PublicTrate ;

struct MyType ;

/* the trait bound `MyType: Sealed` is not satisfied, т.е. Sealed - приватный метод.
 *
impl PublicTrate for MyType {
    fn public_method(&self) {
        println!("Inside ::public_method()") ;
    }
}
 *
 */

pub trait Pretty: Display {
    fn pretty(&self) ->String {
        format!("=> {self}")
    }
}

impl Pretty for i32 {
}

trait Animal {  // Is dyn-compatible
    fn make_sound(&self) ;
}

trait Pet: Animal { // Is dyn-compatible
    fn pet_name(&self) ->String ;
}

struct Dog {
    name:   String
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!") ;
    }
}

impl Pet for Dog {
    fn pet_name(&self) ->String {
        self.name.clone()
    }
}

/*
    ссылка на трейт-объект (trait object) Animal.
    Принимает любой type, который реализует трейт Animal", 
      но при этом размер типа неизвестен на этапе компиляции.
    Расшифровка: &dyn Animal, Animal должет быть dyn !!!!!
      1) & — ссылка на тип
      2) dyn — тип будет известен во время выполнения (vtable (виртуальная таблица))
      3) Animal — trait который должен реализовать тип
*/
fn print_animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

/*
    pet — это ссылка на трейт-объект &dyn Pet. 
    Её внутреннее устройство — это "толстый" указатель:
     - Указатель на данные (ваш объект).
     - Указатель на виртуальную таблицу методов (vtable) для Pet.
    И вот тут главное: поскольку Pet объявлен как trait Pet: Animal, 
       компилятор включает в эту виртуальную таблицу для Pet также и 
       все методы Animal.

    &dyn Pet может работать с методами из Animal, потому что:
     - Связь Pet: Animal гарантирует, что у Pet есть все методы Animal.
     - Виртуальная таблица для &dyn Pet содержит все методы Animal, 
        поэтому вызов возможен.
*/
fn process_pet(pet: &dyn Pet) {
    print_animal_sound(pet);
    println!("Pet name: {}", pet.pet_name()) ;
}

fn main() {
    let x = 42 ;
    x.public_method();  // Out: i32: 42

    let s = "abc".to_string() ;
    s.public_method();  // Out: String: abc

    println!("{}", 10.pretty()) ;   // Out: => 10


    let dog = Dog{name: "my dog.".to_string()} ;
    print_animal_sound(&dog);   // Woof!
    
    process_pet(&dog); /* Out:
    Woof!
    Pet name: my dog.    
     */
}
