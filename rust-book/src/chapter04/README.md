### Capítulo 4 - Ownership

#### Safe
```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}

```

#### Unsafe
```rust

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x);
    let x = true;
}

```
```bash
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:9:10
  |
9 |     read(x);
  |          ^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

> Este segundo programa não é seguro porque read(x)espera xter um valor do tipo bool, mas xainda não tem um valor.
> Quando um programa como esse é executado por um interpretador, então ler x antes de ser definido levantaria uma exceção como a do Python NameError ou Javascript ReferenceError. Mas exceções têm um custo. Cada vez que um programa interpretado lê uma variável, então o interpretador deve verificar se essa variável está definida.

> No Rust essa checagem acontece no momento de compilação, reduzindo ao mínimo as checagens em runtime.

#### Objetivos do Rust

- Um objetivo fundamental do Rust é garantir que seus programas nunca tenham comportamento indefinido.**

- Um objetivo secundário do Rust é evitar comportamento indefinido em tempo de compilação em vez de tempo de execução.


No contexto de Rust (e outras linguagens de programação), entender a diferença entre Heap e Stack é crucial, especialmente considerando o foco da linguagem em segurança de memória e eficiência.

#### Conceitos Gerais: Heap vs. Stack
1. Stack:
    - Uma região da memória usada para armazenar dados locais, variáveis de escopo limitado e execução de funções.
    - Funciona como uma estrutura LIFO (Last In, First Out), ou seja, o último dado a ser adicionado é o primeiro a ser removido.
    - A alocação e desalocação de memória no Stack são automáticas e muito rápidas, pois a ordem é bem definida e controlada em tempo de compilação.
    - Quando uma função é chamada, suas variáveis são empilhadas no Stack, e quando a função termina, essas variáveis são automaticamente removidas.
2. Heap:
    - Uma região da memória usada para armazenar dados que precisam sobreviver além do escopo de uma função ou que têm tamanho desconhecido em tempo de compilação.
    - A alocação de memória no Heap é manual e geralmente mais lenta, pois envolve uma busca por espaço livre.
    - Os dados alocados no Heap não são automaticamente liberados quando uma função termina; é necessário desalocar manualmente ou usar um mecanismo automático, como o Garbage Collector em outras linguagens (Rust usa um modelo diferente para isso, que veremos adiante).

#### Como o Rust Gerencia Heap e Stack
Rust é uma linguagem que não tem um Garbage Collector. Em vez disso, usa um sistema de ownership (propriedade) que garante segurança e gerenciamento eficiente de memória em tempo de compilação.

1. Stack em Rust:

    - Variáveis simples como inteiros, floats, e booleanos (tipos com tamanhos conhecidos em tempo de compilação) são alocadas no Stack.
    - Variáveis que possuem escopo restrito (por exemplo, dentro de uma função) são automaticamente desalocadas assim que saem desse escopo.

```rust
fn main() {
    let x = 10; // 'x' é alocado no Stack
    println!("x: {}", x);
} // 'x' é automaticamente desalocado aqui
```

2. Heap em Rust:

    - Para alocar memória no Heap, usamos o tipo Box, Vec, String, ou outras coleções que requerem um tamanho dinâmico.
    - Quando você aloca memória no Heap em Rust, a variável em si (o ponteiro para o dado) fica no Stack, mas o dado ao qual ele aponta é alocado no Heap.

```rust
fn main() {
    let heap_variable = Box::new(42); // O dado '42' está no Heap, o ponteiro está no Stack
    println!("heap_variable: {}", heap_variable);
} // 'heap_variable' é desalocado automaticamente aqui, liberando o Heap
```

Nesse exemplo, o Rust automaticamente libera a memória do Heap assim que `heap_variable` sai de escopo, graças ao sistema de ownership.

#### Comparação Prática: Quando Usar Stack e Heap
| Característica        |	Stack         |	Heap                                          |
|-----------------------|-----------------|-----------------------------------------------|
| Velocidade	        | Muito rápida	            | Mais lenta                          |
| Tamanho de dados	    | Limitado	                | Flexível (ideal para dados grandes) |
| Alocação/Desalocação	| Automática	            | Manual (ou gerida pelo Rust)        |
| Vida útil dos dados	| Controlada pelo escopo	| Sobrevive fora do escopo            |
| Exemplo em Rust	    | `let x = 5;`	            | `let x = Box::new(5);`              |

#### Conclusão
- Use o Stack para dados de tamanho fixo e que não precisam sobreviver além do escopo de uma função.
- Use o Heap para dados de tamanho variável ou que precisam ser compartilhados entre diferentes partes do programa.
O sistema de ownership em Rust (compondo ownership, borrowing e lifetimes) é o que diferencia Rust de outras linguagens, permitindo gerenciar Heap e Stack de forma eficiente sem sacrificar a segurança de memória.


#### Princípios
- **Box deallocation principle (fully correct)**: If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

- **Moved heap data principle**: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

- **Pointer Safety Principle**: data should never be aliased and mutated at the same time.

#### Resumo
- Toda informação no Heap deve ter apenas uma variável como dono
- Rust desaloca o Heap quando a variável owner sai do escopo
- Ownership pode ser transferida por movimentos que acontecem em atribuições e chamadas de função.
- Dados no Heap podem ser acessados apenas pelo dono atual e não pelos anteriores.
