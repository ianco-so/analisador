struct Analisador {
    entrada: String,
    pos: usize,
    cache: Option<(usize, String)>,
}

impl Analisador {
    fn novo(entrada: String) -> Self {
        Analisador {
            entrada,
            pos: 0,
            cache: None,
        }
    }

    fn proximo(&mut self) -> Result<(String, usize), String> {
        if let Some((pos, s)) = self.cache.take() {
            return Ok((s, pos));
        }

        self.pula_espaco_vazio();

        if self.pos >= self.entrada.len() {
            return Err("Fim da entrada".to_string());
        }

        let pos_inicio = self.pos;
        let letra = self.entrada.chars().nth(self.pos).unwrap();

        if letra.is_digit(10) {
            let mut num_str = String::new();
            while self.pos < self.entrada.len() && self.entrada.chars().nth(self.pos).unwrap().is_digit(10) {
                num_str.push(self.entrada.chars().nth(self.pos).unwrap());
                self.pos += 1;
            }
            Ok((num_str, pos_inicio))
        } else if letra == '+' || letra == '-' {
            self.pos += 1;
            Ok((letra.to_string(), pos_inicio))
        } else {
            Err(format!("Erro na posição {}", self.pos))
        }
    }

    fn pula_espaco_vazio(&mut self) {
        while self.pos < self.entrada.len() && self.entrada.chars().nth(self.pos).unwrap().is_whitespace() {
            self.pos += 1;
        }
    }

}

fn main() {
    use std::io::{self, Write};

    loop {
        print!("Expressão: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        if input.is_empty() {
            break;
        }

        let mut analisador = Analisador::novo(input);
        loop {
            match analisador.proximo() {
                Ok((lexico, pos)) => {
                    print!("(\"{}\", {}) ", lexico, pos);
                }
                Err(err) => {
                    if err == "Fim da entrada" {
                        break;
                    } else {
                        print!("{}", err);
                        break;
                    }
                }
            }
        }
        println!();
    }
}
