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
            Instruction::Affiche => {
                println!("{}", memoire[position])
            }
            Instruction::Lis => {
                let mut input = String::new();
                println!("Entrez un nombre : ");
                std::io::stdin().read_line(&mut input).unwrap();
                memoire[position] = input.trim().parse::<i32>().unwrap();
            }
        }
        compteur += 1;
    }
}

fn main() {
    //Test 1
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

    //Test 2
    let mut mem2 = vec![72, 101, 108, 108, 111];

    let instructions2 = vec![
        Instruction::Affiche,
        Instruction::Droite,
        Instruction::Affiche,
        Instruction::Droite,
        Instruction::Affiche,
        Instruction::Droite,
        Instruction::Affiche,
        Instruction::Droite,
        Instruction::Affiche,
    ];

    interpreteur(&mut mem2, &instructions2);

    //Test 3
    let mut mem3 = vec![72, 101, 108, 108, 111];

    let instructions3 = vec![
        Instruction::Lis,
        Instruction::Plus,
        Instruction::Affiche,
        Instruction::Lis,
        Instruction::Plus,
        Instruction::Affiche,
        Instruction::Lis,
        Instruction::Plus,
        Instruction::Lis,
        Instruction::Plus,
        Instruction::Affiche,
    ];

    interpreteur(&mut mem3, &instructions3);
}
