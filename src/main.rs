use std::io;

// Definição da estrutura para representar as escolhas do jogador
struct Escolha {
    texto: String,
    pontos: i32,
}

fn main() {
    // Variável para armazenar a pontuação do jogador
    let mut pontos = 0;

    println!("Bem-vindo à Floresta Misteriosa!");

    loop {
        // Exibir opções para o jogador e aguardar a escolha
        println!("Escolha sua próxima ação:");
        println!("1. Entrar na caverna escura");
        println!("2. Seguir o caminho iluminado");
        println!("3. Cruzar a ponte frágil");
        println!("4. Descansar à beira do riacho");

        let mut escolha_str = String::new();
        io::stdin().read_line(&mut escolha_str).expect("Falha ao ler entrada");
        let escolha: u32 = match escolha_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Escolha inválida. Tente novamente.");
                continue;
            }
        };

        // Realizar ação com base na escolha do jogador e atualizar pontos
        match escolha {
            1 => {
                println!("Você entrou na caverna escura e encontrou um tesouro escondido!");
                pontos += 50;
            }
            2 => {
                println!("Você seguiu o caminho iluminado e se deparou com um monstro assustador!");
                println!("O monstro te assustou, mas você escapou ileso.");
                pontos += 10;
            }
            3 => {
                println!("Você decidiu cruzar a ponte frágil e caiu no rio abaixo.");
                println!("Felizmente, você conseguiu nadar até a margem.");
                pontos -= 20;
            }
            4 => {
                println!("Você descansou à beira do riacho e recuperou suas energias.");
                pontos += 30;
            }
            _ => {
                println!("Escolha inválida. Tente novamente.");
                continue;
            }
        }

        // Exibir a pontuação atual do jogador
        println!("Pontuação atual: {}", pontos);

        // Verificar se o jogador atingiu uma pontuação alta ou baixa e exibir mensagem correspondente
        if pontos >= 100 {
            println!("Parabéns! Você é um verdadeiro aventureiro!");
        } else if pontos <= -50 {
            println!("Que pena! Suas escolhas te levaram a um final trágico na Floresta Misteriosa.");
        }

        // Perguntar ao jogador se deseja continuar jogando
        println!("Deseja continuar explorando a Floresta Misteriosa? (S/N)");

        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta).expect("Falha ao ler entrada");
        if resposta.trim().to_lowercase() != "s" {
            break;
        }
    }

    println!("Obrigado por jogar 'A Floresta Misteriosa'!");
}
