
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    println!("Hello, world!");
    // panic!("Quebra tudo");//aborta

    let v = vec![1, 2, 3];
    // v[99];//vai quebrar

    let file = File::open("Hello.txt");


    let file = match file {
        Ok(file) =>{
            println!("Arquivo aberto");
            file
        },
        Err (ref error) if error.kind() == ErrorKind::NotFound => {
            println!("Arquivo náo existe, criando");
            match File::create("Hello.txt"){
                    Ok(file) => {
                        println!("Arquivo criado");
                        file
                    },
                    Err(error) =>{
                        panic!("Erro ao criar o arquivo: {:?}", error);
                    }
                }
            },
        Err(error) =>{
            panic!("Erro ao abrir o arquivo: {:?}", error);
        }
    };

    let f = File::open("Hello.txt").unwrap();
    // let f = File::open("hellos.txt").expect("Falhou ao abrir hello.txt");
    let result = read_file_content(String::from("Hello.txt"));
    match result {
        Ok(content) => {
            println!("conteudo do arquivo é: {}", content)
        }Err(error)=> {
            println!("Erro ao abrir o arquivo: {}", error)
        }
    }


}

fn read_file_content(filename: String) -> Result<String,std::io::Error> {
    // let mut file = File::open(filename);
    // match file {
    //     Ok(file)=>file,
    //     Err(error)=> return Err(error)
    // };
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s)?;
   Ok(s)
}