use kinds::*;

/// Combines two primary colors in equal amounts to create
/// a secondary color.
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    // dummy and incomplete code
    match c1 {
        PrimaryColor::Yellow => match c2 { 
            PrimaryColor::Red => SecondaryColor::Orange,
            PrimaryColor::Blue => SecondaryColor::Green,
            _ => SecondaryColor::Orange },        
        _ => SecondaryColor::Purple
    }
}
