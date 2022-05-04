enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
}

fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>) {
    let mut compteur = 0;
    let mut position = 0;
    while compteur < instructions.len() {
        match instructions[compteur] {
            Instruction::Plus => {
                memoire[position] += 1;
            }
            Instruction::Moins => {
                memoire[position] -= 1;
            }
            Instruction::Droite => {
                position += 1;
            }
            Instruction::Gauche => {
                position -= 1;
            }
            Instruction::Affiche => println!("Vide"),
            Instruction::Lis => println!("Vide"),
        }
        compteur += 1;
    }
}

fn main() {
    let mut mem = vec![0, 0, 0];
    let instructions = vec![
        Instruction::Plus,
        Instruction::Plus,
        Instruction::Plus,
        Instruction::Droite,
        Instruction::Plus,
        Instruction::Droite,
        Instruction::Moins,
    ];
    interpreteur(&mut mem, &instructions);

    println!("{:?}", mem);
}
