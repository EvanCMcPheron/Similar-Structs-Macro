# Description
It is not to uncommon when coding in rust to find yourself defining many different structs that all dervie the same trait, are all public, or all have only public feilds. It is often said that you should never repeat yourself when coding, but in situations like those it can seem impossible not to! That is the problem that this tiny crate sets out to solve by providing 2 powerful declaritive macros, similar_structs and similar_enums.
# Usage
To declare structs with similar_structs, simply declare a struct as you usually would within a similar_structs{} but omit the "struct" keyword (you **must use a trailing comma for feilds**, otherwise you will get a cryptic error message!):
```rust
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
Note that individually declaring structs and feilds as public is perfectly valid, although in this situation the macro has a much more efficient solution: you can specify to make all structs, feilds, or both public. These three examples will expand into code that is identical to the code that the snippet above will expand into:
```rust
similar_structs!{
    pub structs

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
    pub feilds

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
    pub all
    
    User {
        credentials: UserCredentials,
    }
    UserCredentials {
        username: String,
        password: String,
    }
}
```
The line clarifying the default visibilty **must** come before any struct definitions.

As for sharing derives between all the declared structs, that can also be done with a single line:
```rust
similar_structs!{
    repeat #[derive(Debug, Clone)]
    pub all
    
    User {
        credentials: UserCredentials,
    }
    UserCredentials {
        username: String,
        password: String,
    }
}
```
In expanded code, this will copy in the derive line for every single struct definitions. This should also work for sharing attributes between structs, but that is untested, and as of now you can only have one "repeat" line per macro. It is also perfectly valid to have the repeat line come after the default visibility line, as long as they both come before any struct definitions. The code snippet above will expand into:
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
Is the difference minor? Yes. However, there is an imrpovement in legibility and repetetiveness, and the difference will only grow as the amount of similar structs declared grows.
