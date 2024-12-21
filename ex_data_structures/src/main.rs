use array_mutation::exercise_array_mutation;
use library::exercise_library;
use store::exercise_store;

// Livraria 2.0: Augmente a livraria feita anteriormente com estruturas de dados
// para eficientemente encontrar um livro pelo seu titulo e encontrar os livros escritos por um autor. 
// Adicionar ISBN e palavras chave a cada livro. Introduzir procura por palavras chave eficiente
// com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

// Mercearia 2.0: Augmente a Merceria para poder mos encontrar eficientemente os produtos dentro de uma fileira, 
// prateleira e zona. Devemos também ser capazes de encontrar eficientemente um produto e a sua posição.

mod library;
mod store;
mod array_mutation;

fn main() {
    exercise_store();
    exercise_library();
    exercise_array_mutation();
}

