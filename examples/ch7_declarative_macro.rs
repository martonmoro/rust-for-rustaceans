// macro_rules! /* macro name */ {
//     (/* 1st matcher */) => { /* 1st transcriber */ };
//     (/* 2nd matcher */) => { /* 2nd transcriber */ };
// }

macro_rules! make_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $val:expr)?),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant $(= $val)?),*
        }

        impl $name {
            pub const ALL: &[Self] = &[$(Self::$variant),*];

            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

make_enum! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Color {
        Red = 1,
        Green,
        Blue = 5,
    }
}

fn main() {
    println!("All colors:");
    for color in Color::ALL {
        println!("  {:?} is called {:?}", color, color.name());
    }

    println!("\nThere are {} colors", Color::ALL.len());

    let search = "Green";
    let found = Color::ALL.iter().find(|&c| c.name() == search);
    println!("\nSearching for '{}': {:?}", search, found);

    let my_color = Color::Blue;
    match my_color {
        Color::Red => println!("\nIt's red!"),
        Color::Green => println!("\nIt's green!"),
        Color::Blue => println!("\nIt's blue!"),
    }

    println!("\nDiscriminant values:");
    for color in Color::ALL {
        println!("  {} = {}", color.name(), *color as i32);
    }
}
