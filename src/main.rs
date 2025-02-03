//DEFINE THE INTERFACE
pub trait DndClass {
    fn attack(&self) -> &'static str;
    fn describe(&self) -> &'static str;
    fn special_ability(&self) -> u32;
}

//CONCRETE IMPLEMENTATION 1
struct Fighter;

impl DndClass for Fighter {
    fn attack(&self) -> &'static str {
        "Main hand attack!"
    }

    fn describe(&self) -> &'static str {
        "Fighters have mastered the art of combat, wielding weapons with unmatched skill and wearing armour like a second skin."
    }

    fn special_ability(&self) -> u32 {
        10
    }
}

//CONCRETE IMPLEMENTATION 2
struct Wizard;

impl DndClass for Wizard {
    fn attack(&self) -> &'static str {
        "Magic missile!"
    }

    fn describe(&self) -> &'static str {
        "Wizards master the arcane by specialising in individual schools of magic, combining ancient spells with modern research."
    }

    fn special_ability(&self) -> u32 {
        8
    }
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_mechanisms() {
        let fighter = Fighter;
        let wizard = Wizard;
        
        assert_eq!(fighter.attack(), "Main hand attack!");
        assert_eq!(wizard.attack(), "Magic missile!");
    }

    #[test]
    fn test_class_descriptions() {
        let fighter = Fighter;
        let wizard = Wizard;
        
        assert!(fighter.describe().contains("Fighter"));
        assert!(wizard.describe().starts_with("Wizard"));
    }

    #[test]
    fn test_special_abilities() {
        let fighter = Fighter;
        let wizard = Wizard;
        
        assert!(fighter.special_ability() >= 10);
        assert!(wizard.special_ability() <= 8);
    }
}

//MAIN
fn main() {
    let party: Vec<Box<dyn DndClass>> = vec![
        Box::new(Fighter),
        Box::new(Wizard),
    ];

    for member in party {
        println!("{}", member.describe());
        println!("Attack: {}", member.attack());
        println!("Special Ability: {}\n", member.special_ability());
    }
}