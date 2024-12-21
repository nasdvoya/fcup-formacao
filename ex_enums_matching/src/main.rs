use geometric_forms::exercise_geometric_forms;
use library::exercise_library;

mod library;
mod geometric_forms;

/// Formas geométricas: Crie structs que representem várias formas geométricas
/// (Quadrado, circulo, elipse, Triângulo, Cubo, Cilindro, Esfera).
/// Depois, utilize os enums para juntar todas as formas em apenas um tipo, Forma.
/// Este tipo deve ser capaz de calcular a área e perímetro e volume.

/// Livraria 3.0: Extender a livraria anterior para poder ter livros, audio books, estátuas e quadros.
/// Deve manter as mesmas capacidades. Deve também ter o máximo de elementos comuns em zonas partilhadas,
/// utilizando a composição para partilhar o máximo de código possivel
/// (por exemplo, todos os elementos têm titulo e autor, mas apenas os audio books têm durações, apenas as estátuas têm dimensões, etc).

fn main() {
    // exercise_library();
    exercise_geometric_forms();
}
