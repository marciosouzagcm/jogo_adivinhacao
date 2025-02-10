# Jogo de Adivinhação

Este é um jogo de adivinhação simples desenvolvido em Rust com uma interface gráfica utilizando a biblioteca `eframe` e `egui`. O objetivo do jogo é adivinhar um número secreto gerado aleatoriamente entre 1 e 100.

## Como Funciona

O jogo gera um número secreto aleatório entre 1 e 100. O usuário deve inserir uma suposição e o jogo irá comparar a suposição com o número secreto, exibindo uma mensagem indicando se a suposição é muito pequena, muito grande ou correta. Se a suposição estiver correta, o jogo reinicia com um novo número secreto.

## Dependências

Para executar este projeto, você precisará das seguintes dependências:

- `rand` para geração de números aleatórios
- `eframe` e `egui` para a interface gráfica

Estas dependências estão especificadas no arquivo `Cargo.toml`:

```toml
[dependencies]
rand = "0.9.0"
egui = "0.31.0"
eframe = "0.31.0"
````

## Como Executar
Para compilar e executar o projeto, siga os passos abaixo:

Clone o repositório para o seu ambiente local.
Navegue até o diretório do projeto.
Execute o comando cargo run no terminal.

git clone https://github.com/marciosouzagcm/jogo_adivinhacao.git
cd jogo_adivinhacao
cargo run

Estrutura do Código
Aqui está uma descrição detalhada do código principal (main.rs):


fn main() {
    // Configura as opções padrão para a aplicação nativa do eframe.
    let options = eframe::NativeOptions::default();
    
    // Executa a aplicação nativa do eframe com o título "Jogo de Adivinhação".
    // A função run_native retorna um Result, então usamos let _ = para ignorar o resultado.
    let _ = eframe::run_native(
        "Jogo de Adivinhação",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))), // Cria uma nova instância de MyApp usando a implementação padrão.
    );
}

// Define a estrutura MyApp que representa o estado da aplicação.
struct MyApp {
    numero_secreto: u32, // Armazena o número secreto que o usuário deve adivinhar.
    suposicao: String, // Armazena a suposição atual do usuário.
    mensagem: String, // Armazena a mensagem a ser exibida na interface.
}

// Implementa o traço Default para a estrutura MyApp.
impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Gera um número secreto aleatório entre 1 e 100.
            numero_secreto: rand::rng().random_range(1..=100),
            suposicao: String::new(), // Inicializa a suposição como uma string vazia.
            mensagem: String::from("Adivinhe o número!"), // Define a mensagem inicial.
        }
    }
}

// Implementa o traço eframe::App para a estrutura MyApp.
impl eframe::App for MyApp {
    // Define a função update que atualiza a interface gráfica.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Cria um painel central na interface.
        egui::CentralPanel::default().show(ctx, |ui| {
            // Exibe a mensagem atual.
            ui.label(&self.mensagem);

            // Cria uma linha horizontal para a entrada de texto.
            ui.horizontal(|ui| {
                ui.label("Sua suposição:"); // Exibe o rótulo "Sua suposição:".
                ui.text_edit_singleline(&mut self.suposicao); // Cria uma caixa de texto para a suposição do usuário.
            });

            // Cria um botão "Enviar" e verifica se foi clicado.
            if ui.button("Enviar").clicked() {
                // Tenta converter a suposição do usuário para um número inteiro.
                let suposicao: u32 = match self.suposicao.trim().parse() {
                    Ok(num) => num, // Se a conversão for bem-sucedida, usa o número.
                    Err(_) => {
                        // Se a conversão falhar, exibe uma mensagem de erro.
                        self.mensagem = String::from("Por favor, insira um número válido!");
                        return; // Sai da função update.
                    }
                };

                // Compara a suposição do usuário com o número secreto.
                match suposicao.cmp(&self.numero_secreto) {
                    Ordering::Less => self.mensagem = String::from("Muito pequeno!"), // Se a suposição for menor, exibe "Muito pequeno!".
                    Ordering::Greater => self.mensagem = String::from("Muito grande!"), // Se a suposição for maior, exibe "Muito grande!".
                    Ordering::Equal => {
                        // Se a suposição for igual, exibe "Você ganhou!" e gera um novo número secreto.
                        self.mensagem = String::from("Você ganhou!");
                        self.numero_secreto = rand::rng().random_range(1..=100);
                    }
                }
            }
        });
    }
}

Contribuição
Se você quiser contribuir para este projeto, sinta-se à vontade para abrir issues e pull requests no repositório GitHub.

Licença
Este projeto está licenciado sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.

