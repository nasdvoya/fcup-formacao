// Livraria 4.0: Devemos ter funções que funcionem para os objetos correspondentes 
// (por exemplo, obter duração de audio book, dimensões de uma estátua), 
// mas que retornem um erro quando tentamos utilizar em objetos que não possuam essas propriedades. 
// Deve também adicionar a capacidade de procurar obras pelo titulo, autor e tipo de obra. 
// Estes métodos devem retornar objetos que podem ser nulos (Opcionais). 

// Merceria 3.0: Augmente a merceria desenvolvida com a utilização de Options e Results. 
// Devemos retornar objetos opcionais quando fazemos lookups potencialmente nulas. 
// Devemos utilizar results quando fazemos ações que podem potencialmente falhar 
// (adicionar e remover items, mover items de um local para outro). Deve ter tipos de erros explícitos e informativos.

use library::exercise_library;
use store::exercise_store;

mod library;
mod store;

fn main() {
    exercise_library();
    exercise_store();
}
