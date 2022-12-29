macro_rules! similar_structs {
    //NO PUBS
    ($($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $($vis struct $name {
            $($fvis $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta]$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    //PUB STRUCTS
    (pub structs$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(pub struct $name {
            $($fvis $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta]pub structs$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    (pub structs repeat #[$derives: meta]$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    //PUB FEILDS
    (pub feilds$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $($vis struct $name {
            $(pub $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta]pub feilds$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    (pub feilds repeat #[$derives: meta]$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    //PUB ALL
    (pub all$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(pub struct $name {
            $(pub $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta]pub all$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    (pub all repeat #[$derives: meta]$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
}

