pub enum Job {
    Neet,
    Carpenter,
    Clafter,
    Thief,
    Rich
}

pub enum Stage {
    Flag,Clock,Stand
}

pub enum Material {
    Black,White
}

pub enum Position {
    Resident(City),
    Resistor(City),
    Symmetrist,
    Designer,
}

pub enum City {
    Morning,Night
} 
