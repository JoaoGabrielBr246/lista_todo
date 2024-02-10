use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;
use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("Lista de tarefas TODO");
    
    let nome_arquivo = "TODO.txt";

    loop {
        println!("0-) Ver lista");
        println!("1-) Adicionar tarefa");
        println!("2-) Deletar todas as tarefas");
        println!("3-) Sair");

        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Erro ao ler a linha");
        let opcao: i8 = op.trim().parse().expect("Digite um número válido!");

        if opcao == 0 {
            match ler_arquivo(nome_arquivo) {
                Ok(conteudo) => {
                    println!("Conteúdo do arquivo:");
                    println!("{}", conteudo);
                },
                Err(e) => println!("Erro ao ler o arquivo: {}", e),
            }
        }

        if opcao == 1 {
            loop {
                println!("Digite a tarefa (ou 's' para sair):");
                let mut conteudo = String::new();
                io::stdin().read_line(&mut conteudo).expect("Erro ao ler a linha");

                if conteudo.trim() == "s" {
                    break;
                }

                match adicionar_tarefa(nome_arquivo, &conteudo) {
                    Ok(_) => {
                        println!("Tarefa adicionada com sucesso!");
                    },
                    Err(e) => println!("Erro ao adicionar a tarefa: {}", e),
                }
            }
        }

        if opcao == 2 {
            if deletar_todas_tarefas(nome_arquivo).is_ok() {
                println!("Todas as tarefas foram deletadas com sucesso!");
            } else {
                println!("Erro ao deletar todas as tarefas.");
            }
        }

        if opcao == 3 {
            println!("Saindo...");
            break;
        }
    }

    Ok(())
}

fn adicionar_tarefa(nome_arquivo: &str, conteudo: &str) -> std::io::Result<()> {
    let mut arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(nome_arquivo)?;

    let linhas_anteriores = ler_arquivo(nome_arquivo).unwrap_or_default().lines().count();

    writeln!(arquivo, "{}-{}", linhas_anteriores + 1, conteudo.trim())?;

    Ok(())
}

fn deletar_todas_tarefas(nome_arquivo: &str) -> std::io::Result<()> {
    let arquivo = File::create(nome_arquivo)?;
    arquivo.set_len(0)?;

    Ok(())
}

fn ler_arquivo(nome_arquivo: &str) -> std::io::Result<String> {
    let mut arquivo = File::open(nome_arquivo)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}
