use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let filename = "hello.txt";
    let data = b"Hello, world!\n";

    // Criar um novo arquivo
    create_file(filename).expect("Erro ao criar o arquivo");

    // Escrever dados no arquivo
    write_to_file(filename, data).expect("Erro ao escrever no arquivo");

    // Ler dados do arquivo
    match read_file(filename) {
        Ok(_) => println!("Leitura concluída."),
        Err(e) => eprintln!("Erro ao ler o arquivo: {}", e),
    }

    // Deletar arquivo
    match delete_file(filename) {
        Ok(_) => println!("Arquivo deletado com sucesso."),
        Err(e) => eprintln!("Erro ao deletar o arquivo: {}", e),
    }
}

// Função para criar um novo arquivo
fn create_file(filename: &str) -> Result<(), io::Error> {
    File::create(filename).map(|_| ())
}

// Função para escrever dados em um arquivo
fn write_to_file(filename: &str, data: &[u8]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;
    file.write_all(data)?;
    Ok(())
}

// Função para ler o conteúdo de um arquivo e exibir no console
fn read_file(filename: &str) -> Result<(), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

// Função para deletar um arquivo
fn delete_file(filename: &str) -> Result<(), io::Error> {
    remove_file(filename)?;
    Ok(())
}
