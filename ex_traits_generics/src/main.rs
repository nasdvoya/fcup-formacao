// Sorting Genérico: Desenvolver dois algoritmos de sorting (bubble, select) 
// que sejam capaz de ordenar qualquer Vec<T>, desde que T: Ord. 
// Não deve usar as funções de sort da standard library. Deve utilizar referências mutáveis.

// Stack Genérica: Utilizar as estruturas de dados faladas para implementar uma stack genérica.

// Merceria 4.0: Incremente a merceria ao definir uma trait que defina os comportamentos necessários de um artigo. 
// A nossa merceria deve ser capaz de ser utilizada para um tipo de item genérico. Devemos manter todas as capacidades anteriores.

use store::exercise_store;

mod store;

fn main() {
    exercise_store();
}
