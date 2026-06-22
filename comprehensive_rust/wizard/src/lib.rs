#![allow(dead_code)]


#[derive(Clone, Debug)]
struct Spell {
    name: String,
    cost: u32,
    uses: u32,
}

#[derive(Debug)]
struct Wizard {
    spells: Vec<Spell>,
    mana: u32,
}

impl Wizard {
    fn new(mana: u32) -> Self {
        Wizard { spells: vec![], mana }
    }

    // take ownership of a spell and add it to the wizard's inventory.
    fn add_spell(&mut self, spell: Spell) {
        for s in &self.spells {
            if s.name == spell.name {
                println!("There is already a spell with name {}: {:?}", spell.name, spell);
                return;
            }
        }
        self.spells.push(spell);
    }

    // borrow a spell from the inventory and cast it. The wizard's mana
    // should decrease by the spell's cost and the number of uses for
    // the spell should decrease by 1.
    //
    // If the wizard doesn't have enough mana, the spell should fail.
    // If the spell has no uses left, it is removed from the inventory.
    fn cast_spell(&mut self, name: &str) {
        let mut idx = None;

        for (i, spell) in self.spells.iter_mut().enumerate() {
            if spell.name == name {
                idx = Some(i);
                break;
            }
        }

        let Some(i) = idx else {
            println!("\x1b[37mspell not found: {name}\x1b[0m");
            return;
        };

        let spell = &mut self.spells[i];
        println!("\x1b[37mfound spell: {:?}\x1b[0m", spell);

        if self.mana < spell.cost {
            println!("\x1b[37mnot enough mana {} in {}\x1b[0m", self.mana, spell.cost);
            return;
        }

        if spell.uses < 1 {
            println!("\x1b[37mno uses left: {}\x1b[0m", spell.uses);
            return;
        }

        self.mana -= spell.cost;

        spell.uses -= 1;
        if spell.uses == 0 {
            println!("\x1b[37mremoving spell {:?}\x1b[0m", spell);
            self.spells.remove(i);
        }
    }
}

fn main() {
    let mut merlin = Wizard::new(100);
    let fireball = Spell { name: String::from("Fireball"), cost: 10, uses: 2 };
    let ice_blast = Spell { name: String::from("Ice Blast"), cost: 15, uses: 1 };

    merlin.add_spell(fireball);
    merlin.add_spell(ice_blast);

    merlin.cast_spell("Fireball"); // Casts successfully
    merlin.cast_spell("Ice Blast"); // Casts successfully, then removed
    merlin.cast_spell("Ice Blast"); // Fails (not found)
    merlin.cast_spell("Fireball"); // Casts successfully, then removed
    merlin.cast_spell("Fireball"); // Fails (not found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_spell() {
        let mut wizard = Wizard::new(10);
        let spell = Spell { name: String::from("Fireball"), cost: 5, uses: 3 };
        wizard.add_spell(spell);
        assert_eq!(wizard.spells.len(), 1);
    }

    #[test]
    fn test_cast_spell() {
        let mut wizard = Wizard::new(10);
        let spell = Spell { name: String::from("Fireball"), cost: 5, uses: 3 };
        wizard.add_spell(spell);

        wizard.cast_spell("Fireball");
        assert_eq!(wizard.mana, 5);
        assert_eq!(wizard.spells.len(), 1);
        assert_eq!(wizard.spells[0].uses, 2);
    }

    #[test]
    fn test_cast_spell_insufficient_mana() {
        let mut wizard = Wizard::new(10);
        let spell = Spell { name: String::from("Fireball"), cost: 15, uses: 3 };
        wizard.add_spell(spell);

        wizard.cast_spell("Fireball");
        assert_eq!(wizard.mana, 10);
        assert_eq!(wizard.spells.len(), 1);
        assert_eq!(wizard.spells[0].uses, 3);
    }

    #[test]
    fn test_cast_spell_not_found() {
        let mut wizard = Wizard::new(10);
        wizard.cast_spell("Fireball");
        assert_eq!(wizard.mana, 10);
    }

    #[test]
    fn test_cast_spell_removal() {
        let mut wizard = Wizard::new(10);
        let spell = Spell { name: String::from("Fireball"), cost: 5, uses: 1 };
        wizard.add_spell(spell);

        wizard.cast_spell("Fireball");
        assert_eq!(wizard.mana, 5);
        assert_eq!(wizard.spells.len(), 0);
    }

    #[test]
    fn test_add_spell_twice() {
        let mut wizard = Wizard::new(10);
        let spell = Spell { name: String::from("Fireball"), cost: 5, uses: 1 };
        wizard.add_spell(spell.clone());
        assert_eq!(wizard.spells.len(), 1);

        wizard.add_spell(spell);
        assert_eq!(wizard.spells.len(), 1);
    }
}
