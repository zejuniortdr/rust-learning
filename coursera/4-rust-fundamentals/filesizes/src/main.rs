use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Verifica se foi passado o argumento esperado
    if args.len() < 2 {
        eprintln!("Por favor, forneça o tamanho como argumento.");
        return;
    }

    // O primeiro argumento é o valor numérico, o segundo é a unidade
    let size_str = &args[1];
    let parts: Vec<&str> = size_str.split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Formato inválido! Esperado 'valor unidade'.");
        return;
    }

    let value: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Erro ao analisar o valor numérico.");
            return;
        }
    };

    let unit = parts[1].to_lowercase();

    // Converte o valor conforme a unidade informada
    let (bytes, kilobytes, megabytes, gigabytes) = match unit.as_str() {
        "mb" => {
            let bytes = value * 1_000_000.0;
            let kilobytes = bytes / 1_000.0;
            let megabytes = bytes / 1_000_000.0;
            let gigabytes = bytes / 1_000_000_000.0;
            (bytes, kilobytes, megabytes, gigabytes)
        }
        "kb" => {
            let bytes = value * 1_000.0;
            let kilobytes = bytes / 1_000.0;
            let megabytes = bytes / 1_000_000.0;
            let gigabytes = bytes / 1_000_000_000.0;
            (bytes, kilobytes, megabytes, gigabytes)
        }
        "bytes" => {
            let bytes = value;
            let kilobytes = bytes / 1_000.0;
            let megabytes = bytes / 1_000_000.0;
            let gigabytes = bytes / 1_000_000_000.0;
            (bytes, kilobytes, megabytes, gigabytes)
        }
        "gb" => {
            let bytes = value * 1_000_000_000.0;
            let kilobytes = bytes / 1_000.0;
            let megabytes = bytes / 1_000_000.0;
            let gigabytes = bytes / 1_000_000_000.0;
            (bytes, kilobytes, megabytes, gigabytes)
        }
        _ => {
            eprintln!("Unidade desconhecida: {}", unit);
            return;
        }
    };

    // Exibe o resultado formatado
    println!(
        "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} KB\", megabytes: \"{} MB\", gigabytes: \"{} GB\" }}",
        bytes, kilobytes, megabytes, gigabytes
    );
}
