macro_rules! add {
    (Чш, я събери ($var1:expr) и ($var2:expr)) => {
        $var1 + $var2
    }
}

macro_rules! map {
    {
        $( $key: expr => $value: expr ),*$(,)?
    } => {
        {
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    }
}

//Macros in rust are hygenic

fn main() {
    println!("{}", add!(Чш, я събери (1) и (1)));

    let m = map! {
    "a" => 1,
    "b" => 2,
    };

    println!("{:?}", m);

    //Macros in rust are hygienic
    macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

    println!("{}", five_times!(2 + 3));

    //You need to pass variable name
    macro_rules! foo {
    ($v:ident) => (let $v = 3;);
}
    foo!(x);
    println!("{}",x);

    //Html shorthand
    use std::fmt::Write;
    macro_rules! write_html {
    ($w: expr, ) => (());

    ($w: expr, $e: tt) => (write!($w, "{}", $e)?);

    ($w: expr, $tag: ident [ $( $inner: tt )* ] $( $rest: tt )*) => {{
        write!($w, "<{}>", stringify!($tag))?;
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag))?;
        write_html!($w, $($rest)*);
    }};
}

    let mut out = String::new();

    write_html! {
    &mut out,
    html[
        head[title["Macros guide"]]
        body[h1["Macros are the best!"]]
    ]
}

    println!("{}", out);
}
