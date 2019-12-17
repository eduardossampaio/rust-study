pub trait Summary {
    fn resumo(&self) -> String;

    fn msg(&self) -> String {
        String::from("Mensagem padrão")
    }
}

pub struct Noticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub content: String,
};

impl Summary for Noticia {
    fn resumo(&self) -> String {
        format!("{}, by {} ({})", self.titulo, self.autor, self.local)
    }
};

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn resumo(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let lista_numero = vec![34, 50, 25, 132, 100, 65];

    // let mut maior = lista_numero[0];

    // for numero in lista_numero {
    //     if numero > maior {
    //         maior = numero;
    //     }
    // }

    // println!("O maior número é {}", maior);
    let maior = maior_numero(&lista_numero);
    // let maior = maior_valor(&lista_numero);
    println!("O maior número é {}", maior);

    let chars = vec!['1', 'a', 'g', 'd', 'n', 'y'];
    let maior = maior_char(&chars);
    // let maior = maior_valor(&chars);
    println!("O maior char  é {}", maior);

    struct Ponto<T> {
        x: T,
        y: T,
    }

    impl Ponto<f32> {
        fn distancia_da_origem(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<T> Ponto<T> {
        fn double_x(&self) -> &T {
            // self.x * 2
            &self.x
        }
    }
    let point1 = Ponto { x: 1, y: 4 };
    println!("valor de x: {}, dobro: {}", point1.x, point1.double_x());
    let point1 = Ponto { x: 1.0, y: 4.2 };
    println!("distancia origem : {}", point1.distancia_da_origem());



    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("claro, como vocês provavelmente já sabem,pessoas"),
        reply: false,
        retweet: false,
    };

    let noticia = Noticia {
        titulo: String::from("Noticia interessante"),
        local: String::from("Brasília"),
        autor: String::from("Eduardo"),
        content: String::from("Algo muito interessante aconteceu..."),
    };

    println!("Tweet: {}", tweet.resumo());
    println!("Noticia: {}", noticia.resumo());

    print_resumo(tweet);
    print_resumo(noticia);

    let maior = maior_valor(&lista_numero);
    println!("O maior número é {}", maior);
    let maior = maior_valor(&chars);
    println!("O maior char  é {}", maior);
}

fn print_resumo<T: Summary>(obj: T) {
    println!("Resumo: {}", obj.resumo());
}

fn maior_numero(numeros: &[i32]) -> i32 {
    let mut maior = numeros[0];
    for &numero in numeros {
        if numero > maior {
            maior = numero;
        }
    }
    return maior;
}

fn maior_char(chars: &[char]) -> char {
    let mut maior = chars[0];
    for &char_ in chars {
        if char_ > maior {
            maior = char_;
        }
    }
    maior
}
use std::cmp::PartialOrd;

fn maior_valor<T: PartialOrd + Copy>(valores: &[T]) -> T {
    let mut maior = valores[0];
    for &val in valores.iter() {
        if val > maior {
            maior = val;
        }
    }
    maior
}

fn some_function<T, U>(t: T, u: U) -> u32
where
    T: PartialOrd + Copy,
    U: PartialOrd + Copy,
{
    32
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}