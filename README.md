# similar_structs_macros
This crate provides two declarative macros to help avoid repetition when defining structs and enums that share certain traits, attributes, or visibility. The similar_structs! macro allows you to declare multiple structs at once and specify that all structs, fields, or both should be public by default. It also has the option to specify that all structs should derive certain traits or attributes by default. The similar_enums! macro functions similarly, but for enums instead of structs.
# Usage
To declare structs with similar_structs, simply declare a struct, as you usually would, within a similar_structs!{}, but omit the "struct" keyword (you **must use a trailing comma for fields**, otherwise you will get a cryptic error message!):
```rust
use similar_structs_macros::similar_structs;

similar_structs!{
    pub User {
        pub credentials: UserCredentials,
    }
    pub UserCredentials {
        pub username: String,
        pub password: String,
    }
}
```
Note that individually declaring structs and fields as public is perfectly valid, However you can specify that all structs, fields, or both should be public by default like in the following examples that give identical outputs:
```rust
similar_structs!{
    pub structs;

    User {
        pub credentials: UserCredentials,
    }
    UserCredentials {
        pub username: String,
        pub password: String,
    }
}
```
```rust
similar_structs!{
    pub fields;

    pub User {
        credentials: UserCredentials,
    }
    pub UserCredentials {
        username: String,
        password: String,
    }
}
```
```rust
similar_structs!{
    pub all;
    
    User {
        credentials: UserCredentials,
    }
    UserCredentials {
        username: String,
        password: String,
    }
}
```
The line clarifying the default visibility **must** come before any struct definitions and **must** end with a semicolon.

You can also specify that all structs should derive certain traits or attributes by default with a "repeat #[derive(...)];" line:
```rust
similar_structs!{
    repeat #[derive(Debug, Clone)];
    pub all;
    
    User {
        credentials: UserCredentials,
    }
    UserCredentials {
        username: String,
        password: String,
    }
}
```
This will copy the #[derive(Debug, Clone)] line for every struct definition. It should also work for sharing attributes between structs, although this has not been tested. You can only have one repeat line per macro, and it must end with a semicolon. The repeat line can come before or after the default visibility line, as long as they both come before any struct definitions.
The similar_enums! macro functions similarly to similar_structs!, but for enums instead of structs, and only has the option to specify that all enums should be public by default:
```rust
use similar_structs_macros::similar_enums;

similar_enums!{
    pub enums;
    repeat #[derive(Clone, Debug)];

    State {
        Alive(usize),
        Dead {
            is_buried: bool,
            is_cremated: bool,
        },
    }
    Color {
        Blue,
        Grey,
        Black,
    }
}
```
Here's an example of what the macros expand into

The similar_structs! example above expands into:
```rust
#[derive(Debug, Clone)]
pub struct User {
    pub credentials: UserCredentials,
}
#[derive(Debug, Clone)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}
```
The similar_enums! example above expands into:
```rust
#[derive(Clone, Debug)]
pub enum State {
    Alive(usize),
    Dead {
        is_buried: bool,
        is_cremated: bool,
    }
}
#[derive(Clone, Debug)]
pub enum Color {
    Blue,
    Grey,
    Black,
}
```
While the difference is minor, the declarations using the macros are less cluttered, more concise, and most importantly less repetative.