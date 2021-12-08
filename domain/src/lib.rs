#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub enum Action {
    CarryIn,Build,Steal
}

pub enum Stage {
    Flag,Clock,Stand
}

pub enum Material {
    Black,White
}

pub enum Role {
    Official(City),Resident(City),Symmetrist,Clockmaker
}

pub enum City {
    Morning,Night
} 
