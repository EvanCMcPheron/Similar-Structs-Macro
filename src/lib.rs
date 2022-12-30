macro_rules! similar_structs {
    //NO PUBS
    ($($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $($vis struct $name {
            $($fvis $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta];$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    //pub structs;
    (pub structs;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(pub struct $name {
            $($fvis $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta];pub structs;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    (pub structs; repeat #[$derives: meta];$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $($fvis $fname: $ftype,)*
        }
        )*
    };
    //pub fields;
    (pub fields;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $($vis struct $name {
            $(pub $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta];pub fields;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    (pub fields; repeat #[$derives: meta];$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        $vis struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    //pub all;
    (pub all;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(pub struct $name {
            $(pub $fname: $ftype,)*
        })*
    };
    (repeat #[$derives: meta];pub all;$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
    (pub all; repeat #[$derives: meta];$($vis: vis $name: ident {$($fvis:vis $fname: ident: $ftype: ty,)*})*) => {
        $(
        #[$derives]
        pub struct $name {
            $(pub $fname: $ftype,)*
        }
        )*
    };
}

macro_rules! similar_enums {
    ($($vis: vis $name: ident {$($varient: ident$(($($v1type: ty)*))?$({$($vname: ident: $v2type: ty,)*})?,)*})*) => {
        $(
        $vis enum $name {
            $($varient$(($($v1type, )*))*$({$($vname: $v2type, )*})*,)*
        }
        )*
    };
    (repeat #[$derives: meta]; $($vis: vis $name: ident {$($varient: ident$(($($v1type: ty)*))?$({$($vname: ident: $v2type: ty,)*})?,)*})*) => {
        $(
        #[$derives]
        $vis enum $name {
            $($varient$(($($v1type, )*))*$({$($vname: $v2type, )*})*,)*
        }
        )*
    };
    //PUB ENUMS (feilds & all are not required, as varients are by default public)
    (pub enums; $($vis: vis $name: ident {$($varient: ident$(($($v1type: ty)*))?$({$($vname: ident: $v2type: ty,)*})?,)*})*) => {
        $(
        pub enum $name {
            $($varient$(($($v1type, )*))*$({$($vname: $v2type, )*})*,)*
        }
        )*
    };
    (pub enums; repeat #[$derives: meta]; $($vis: vis $name: ident {$($varient: ident$(($($v1type: ty)*))?$({$($vname: ident: $v2type: ty,)*})?,)*})*) => {
        $(
        #[$derives]
        pub enum $name {
            $($varient$(($($v1type, )*))*$({$($vname: $v2type, )*})*,)*
        }
        )*
    };
    (repeat #[$derives: meta]; pub enums; $($vis: vis $name: ident {$($varient: ident$(($($v1type: ty)*))?$({$($vname: ident: $v2type: ty,)*})?,)*})*) => {
        $(
        #[$derives]
        pub enum $name {
            $($varient$(($($v1type, )*))*$({$($vname: $v2type, )*})*,)*
        }
        )*
    };
}
