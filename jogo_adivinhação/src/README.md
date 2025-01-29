# Este código contém muitas informações, então vamos analisá-lo linha por linha.

## Para obter a entrada do usuário e então imprimir o resultado como saída, precisamos trazer a iobiblioteca de entrada/saída para o escopo. A iobiblioteca vem da biblioteca padrão, conhecida como std:

        use std::io;

## Usar a std::iobiblioteca fornece a você uma série de recursos úteis, incluindo a capacidade de aceitar entrada do usuário.

## Como você viu no Capítulo 1, a mainfunção é o ponto de entrada no programa:

        fn main() {

## A fn sintaxe declara uma nova função; os parênteses, (), indicam que não há parâmetros; e as chaves, {, iniciam o corpo da função.

## Como você também aprendeu, println!é uma macro que imprime uma string na tela:

        println!("Adivinhe o número!");

        println!("Por favor, insira seu palpite.");

## Este código está imprimindo um prompt informando qual é o jogo e solicitando informações do usuário.

# Armazenando valores com variáveis.

## Criaremos uma variável para armazenar a entrada do usuário, assim:

        let mut numero = String::new();

## Em Rust, as variáveis ​​são imutáveis ​​por padrão, o que significa que, uma vez que damos um valor à variável, o valor não mudará. Para tornar uma variável mutável, adicionamos mut antes do nome da variável:

## let mut numero introduzirá uma variável mutável chamada numero. O sinal de igual ( =) diz ao Rust que queremos vincular algo à variável agora. À direita do sinal de igual está o valor que numero está vinculado a, que é o resultado da chamada String::new, uma função que retorna uma nova instância de um String. String é um tipo de string fornecido pela biblioteca padrão que é um bit de texto codificado em UTF-8 e expansível.

## A ::sintaxe na ::new linha indica que new é uma função associada do String tipo. Uma função associada é uma função que é implementada em um tipo, neste caso String. Esta new função cria uma nova string vazia. Você encontrará uma new função em muitos tipos porque é um nome comum para uma função que cria um novo valor de algum tipo.

## Na íntegra, a let mut numero = String::new(); linha criou uma variável mutável que está atualmente vinculada a uma nova instância vazia de a String.

# Recebendo entrada do usuário:

## Incluímos a funcionalidade de entrada/saída da biblioteca padrão use std::io; na primeira linha do programa. Agora, chamaremos a stdinfunção do io módulo, o que nos permitirá manipular a entrada do usuário:

        io::stdin()
                .read_line(&mut numero)

## A stdin função retorna uma instância de std::io::Stdin, que é um tipo que representa um handle para a entrada padrão do seu terminal.

## Em seguida, a linha .read_line(&mut numero)chama o read_line método no identificador de entrada padrão para obter a entrada do usuário. Também estamos passando &mut numero como argumento para read_line para dizer em qual string armazenar a entrada do usuário. O trabalho completo de read_line é pegar o que o usuário digitar na entrada padrão e anexá-lo a uma string, então passamos essa string como um argumento. O argumento da string precisa ser mutável para que o método possa alterar o conteúdo da string.

## O & indica que esse argumento é uma referência , o que lhe dá uma maneira de deixar várias partes do seu código acessarem um pedaço de dados sem precisar copiar esses dados para a memória várias vezes. Referências são um recurso complexo, e uma das principais vantagens do Rust. Assim como as variáveis, as referências são imutáveis ​​por padrão. Portanto, você precisa escrever &mut numero em vez de &numero torná-lo mutável.

# Lidando com falhas potenciais com Result:

        .expect("Falha ao ler a linha");

## Result As variantes de são Ok e Err. A Ok variante indica que a operação foi bem-sucedida, e dentro dela Ok está o valor gerado com sucesso. A Err variante significa que a operação falhou, e Err contém informações sobre como ou por que a operação falhou.

## Valores do Result tipo, como valores de qualquer tipo, têm métodos definidos neles. Uma instância de Result tem um expect método que você pode chamar. Se essa instância de Result for um Err valor, expect fará com que o programa trave e exiba a mensagem que você passou como argumento para expect. Se o read_line método retornar um Err, provavelmente seria o resultado de um erro vindo do sistema operacional subjacente. Se essa instância de Result for um Ok valor, expect pegará o valor de retorno que Ok está segurando e retornará apenas esse valor para você, para que você possa usá-lo. Nesse caso, esse valor é o número de bytes na entrada do usuário.

# Imprimindo valores com println!

        println!("Você adivinhou: {}", numero);

## Esta linha imprime a string que agora contém a entrada do usuário. O {}conjunto de chaves é um espaço reservado: pense {}como pequenas pinças de caranguejo que seguram um valor no lugar.