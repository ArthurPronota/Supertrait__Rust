# Supertrait в Rust

## Что такое supertrait

**Supertrait** (супертрейт) — это **требование**, что **любой** тип, реализующий **данный** trait, **обязан** также реализовать **другой** trait.

**Синтаксис:**

```rust
trait Child: Parent {
    // ...
}
```

- **`Child`** — **субтрейт** (subtrait).
- **`Parent`** — **супертрейт** (supertrait).
- **`Child: Parent`** — «любой `Child` **обязан** реализовать `Parent`».

## Базовый пример

```rust
trait Animal {
    fn make_sound(&self);
}

trait Pet: Animal {
    fn pet_name(&self) -> String;
}
```

- **`Animal`** — **базовый** trait.
- **`Pet: Animal`** — **`Pet`** **требует** `Animal`.
- **Любой** тип, реализующий `Pet`, **обязан** реализовать `Animal`.

### Реализация

```rust
struct Dog {
    name: String,
}

// ✅ Обязательно
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// ✅ Обязательно
impl Pet for Dog {
    fn pet_name(&self) -> String {
        self.name.clone()
    }
}
```

### Ошибка: **забыли** `Animal`

```rust
struct Cat;

// ❌ Только Pet, без Animal
impl Pet for Cat {
    fn pet_name(&self) -> String {
        "Cat".to_string()
    }
}
```

**Ошибка:**

```
error[E0277]: the trait bound `Cat: Animal` is not satisfied
  |
5 | impl Pet for Cat {
  |      ^^^ the trait `Animal` is not implemented for `Cat`
```

**Компилятор** требует **все** supertraits.

## Зачем нужны supertraits

### 1. **Гарантия** методов родителя

```rust
fn process_pet(pet: &dyn Pet) {
    // ✅ Метод Animal доступен — гарантирован supertrait
    pet.make_sound();
    println!("{}", pet.pet_name());
}
```

**Внутри `Pet`** можно **вызывать** методы `Animal` — потому что **любой** `Pet` **обязан** быть `Animal`.

### 2. **`&dyn Pet` включает методы `Animal`**

**Vtable** для `&dyn Pet` содержит **все** методы `Animal` **и** `Pet`:

```
+--------------------+
| Animal::make_sound |
+--------------------+
| Pet::pet_name      |
+--------------------+
```

**Поэтому** `&dyn Pet` **можно** передать в `fn(&dyn Animal)`:

```rust
fn print_animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn process_pet(pet: &dyn Pet) {
    print_animal_sound(pet);   // ✅ Pet → Animal (upcast)
    pet.pet_name();
}
```

**Upcast** `&dyn Pet` → `&dyn Animal` **работает** благодаря supertrait.

### 3. **Сужение** множества реализаций

```rust
trait Pet: Animal { ... }
```

**Только** те типы, которые **уже** реализовали `Animal`, **могут** реализовать `Pet`.

**Пример:** нельзя реализовать `Pet` для типа, который **не** `Animal`.

### 4. **Расширение** логики

```rust
trait Animal {
    fn make_sound(&self);
}

trait Pet: Animal {
    fn pet_name(&self) -> String;

    // Дефолтный метод использует Animal
    fn greet(&self) -> String {
        format!("{} says {}", self.pet_name(), self.make_sound())
    }
}
```

**`Pet`** **расширяет** `Animal` **новыми** методами, используя **базовые**.

## Sealed Trait Pattern

**Классическое** применение supertrait — **запрет** внешних реализаций.

### Проблема

```rust
pub trait PublicTrait {
    fn method(&self);
}
```

**Любой** может реализовать `PublicTrait`:

```rust
// В другом крейте
impl PublicTrait for MyType { ... }
```

**Иногда** это **нежелательно** — вы хотите **контролировать**, **кто** реализует.

### Решение: sealed trait

```rust
mod my_lib {
    // Приватный trait — не экспортируется
    trait Sealed { }

    // Публичный trait с приватным supertrait
    pub trait PublicTrait: Sealed {
        fn public_method(&self);
    }

    // Реализуем Sealed только для разрешённых типов
    impl Sealed for i32 { }
    impl Sealed for String { }

    // Реализуем PublicTrait
    impl PublicTrait for i32 {
        fn public_method(&self) {
            println!("i32: {}", self);
        }
    }

    impl PublicTrait for String {
        fn public_method(&self) {
            println!("String: {}", self);
        }
    }
}
```

### Что происходит

**Снаружи** модуля:

```rust
use my_lib::PublicTrait;

struct MyType;

// ❌ Нельзя — Sealed приватный
impl PublicTrait for MyType {
    fn public_method(&self) { }
}
```

**Ошибка:**

```
error[E0277]: the trait bound `MyType: Sealed` is not satisfied
```

**Причина:** `MyType` **не** реализует `Sealed`, а `Sealed` — **приватный** → **нельзя** реализовать **снаружи**.

### Плюсы

- **Контроль** — только **разрешённые** типы.
- **Обратная совместимость** — можно **добавлять** методы.
- **Инкапсуляция** — внутренние детали **скрыты**.

## Разбор вашего примера

### 1. Sealed trait

```rust
mod my_lib {
    trait Sealed { }
    pub trait PublicTrait: Sealed { ... }
    impl Sealed for i32 { }
    impl Sealed for String { }
    impl PublicTrait for i32 { ... }
    impl PublicTrait for String { ... }
}
```

- **`Sealed`** — **приватный**.
- **`PublicTrait: Sealed`** — **supertrait**.
- **Снаружи** — только `i32` и `String` **могут** реализовать.

### 2. `Pretty: Display`

```rust
pub trait Pretty: Display {
    fn pretty(&self) -> String {
        format!("=> {self}")
    }
}

impl Pretty for i32 { }   // ✅ i32: Display
```

- **`Pretty: Display`** — **supertrait**.
- **Дефолтный** метод `pretty` использует `Display`.
- **`i32`** реализует `Display` → **может** реализовать `Pretty`.

### 3. `Pet: Animal`

```rust
trait Animal {
    fn make_sound(&self);
}

trait Pet: Animal {
    fn pet_name(&self) -> String;
}

struct Dog { name: String }

impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

impl Pet for Dog {
    fn pet_name(&self) -> String { self.name.clone() }
}
```

- **`Pet: Animal`** — supertrait.
- **`Dog`** реализует **оба**.
- **`&dyn Pet`** можно **передать** в `fn(&dyn Animal)`.

### 4. Upcast `Pet` → `Animal`

```rust
fn print_animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn process_pet(pet: &dyn Pet) {
    print_animal_sound(pet);   // ✅ Upcast
    println!("Pet name: {}", pet.pet_name());
}
```

**`&dyn Pet`** **автоматически** приводится к **`&dyn Animal`**.

## Сводная таблица

| Supertrait | Subtrait | Что гарантирует |
|---|---|---|
| `Animal` | `Pet` | `Pet` → `Animal` |
| `Display` | `Pretty` | `Pretty` → `Display` |
| `Sealed` | `PublicTrait` | Контроль реализаций |
| `Error` | `MyError` | `MyError` → `Error` |

## Сводная таблица применений

| Применение | Пример |
|---|---|
| **Гарантия** методов родителя | `Pet: Animal` |
| **Upcast** `&dyn Subtrait` → `&dyn Supertrait` | `&dyn Pet` → `&dyn Animal` |
| **Расширение** логики | `Pretty: Display` |
| **Sealed trait** | `PublicTrait: Sealed` |
| **Сужение** множества | Только `Animal` могут быть `Pet` |

## Итог

- **Supertrait** — **требование**, что тип, реализующий **субтрейт**, **обязан** реализовать **супертрейт**.
- **Синтаксис:** `trait Child: Parent`.
- **Зачем:**
  - **гарантия** методов родителя;
  - **upcast** `&dyn Child` → `&dyn Parent`;
  - **сужение** множества реализаций;
  - **расширение** логики;
  - **sealed trait** — **запрет** внешних реализаций.
- **Sealed trait pattern:** **приватный** supertrait → **снаружи** нельзя реализовать.
- **Upcast** `&dyn Pet` → `&dyn Animal` **работает** благодаря supertrait.
- **В вашем примере:**
  - `Sealed` → контроль;
  - `Pretty: Display` → расширение;
  - `Pet: Animal` → upcast и гарантия.
- **Правило:** supertrait **гарантирует**, что **все** реализации субтрейта **имеют** методы супертрейта.
