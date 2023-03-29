use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    println!("Bem vindo ao jogo de HangMan!");

    let mut jogo = HangManGame::new();
    jogo.jogar();
}

struct HangManGame{
    palavra_secreta: String,
    palavra_chutada: String,
    tentativas_incorretas: u8,
    letras_chutadas: Vec<char>,
}

impl HangManGame {

    fn new() -> Self {
        let palavras = ler_palavras();
        let palavra_secreta = palavras[rand::random::<usize>() % palavras.len()].to_lowercase();
        let palavra_chutada= String::with_capacity(palavra_secreta.len());
        let tentativas_incorretas = 0;
        let letras_chutadas = vec![];

        Self {
            palavra_secreta,
            palavra_chutada,
            tentativas_incorretas,
            letras_chutadas,
        }
    }

    fn jogar(&mut self) {

        loop{
            self.print_estado_do_jogo();

            if let Some(ganhador) = self.verificar_ganhador() {
                if ganhador {
                    println!("Você ganhou!");
                } else {
                    println!("Você perdeu! A palavra secreta era '{}'", self.palavra_secreta);
                }
                return;
            }

            print!("Chute uma letra: ");
            io::stdout().flush().expect("Erro ao descarregar o stdout.");

            let chute = ler_chute();
            self.atualizar_estado_do_jogo(chute);
        }
    }

    fn print_estado_do_jogo(&self) {
        println!("Palavras chutadas: {:?}", self.palavra_chutada);
        println!("Chutes incorretos: {}", self.tentativas_incorretas);
        println!("Letras chutadas: {:?}", self.letras_chutadas);
    }

    fn verificar_ganhador(&self) -> Option<bool> {
        if self.tentativas_incorretas >= 5 {
            return Some(false);
        }

        if self.palavra_chutada.eq(&self.palavra_secreta) {
            return Some(true);
        }

        None
    }

    fn atualizar_estado_do_jogo(&mut self, chute: char) {
        if self.letras_chutadas.contains(&chute) {
            println!("Você já chutou essa letra!");
            return;
        }

        self.letras_chutadas.push(chute);

        if self.palavra_secreta.contains(chute) {
            for(i, c) in self.palavra_secreta.chars().enumerate() {
                if c == chute {
                    if i > self.palavra_chutada.len() {
                        self.palavra_chutada.push(chute);
                    } else {
                        self.palavra_chutada.insert(i, chute);
                    }
                }
            }
        } else {
            self.tentativas_incorretas += 1;
        }
    }
}

fn ler_chute() -> char {
    loop {
        let mut chute = String::new();
        io::stdin()
            .read_line(&mut chute)
            .expect("Falha ao ler a entrada.");

        match chute.trim().chars().next() {
            Some(c) if c.is_ascii_alphabetic() => return c.to_ascii_lowercase(),
            Some(_) => println!("Por favor, insira uma letra valida."),
            None => println!("Por favor, insira uma letra.")
        }
    }
}

fn ler_palavras() -> Vec<String> {
    println!("Qual grupo de palavras você deseja jogar? (1 - Frutas, 2 - Animais, 3 - Cores): ");

    let grupo_de_palavras = match_grupo();

    let mut palavras = vec![];
    let mut arquivo = File::open(grupo_de_palavras).expect("Arquivo não encontrado.");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Erro ao ler o arquivo.");

    for palavra in conteudo.split_whitespace() {
        palavras.push(palavra.to_lowercase());
    }

    return palavras;
}

fn match_grupo() -> String {

    let mut grupo_de_palavras = String::new();

    loop {
        io::stdin()
            .read_line(&mut grupo_de_palavras)
            .expect("Falha ao ler a entrada.");

        match grupo_de_palavras.trim() {
            "1" => return "frutas.txt".to_string(),
            "2" => return "animais.txt".to_string(),
            "3" => return "cores.txt".to_string(),
             _  => println!("Por favor, insira um grupo valido.")
        }
    }
}

