use cosmwasm_std::{Addr, Uint256};

pub struct User {
    pub tickets: Vec<Ticket>,
    pub addr: Addr,
}

pub struct Ticket {
    pub owner: User,
    pub value: i32,
}

fn random_ticket_number() -> i32 {
    8
}

pub fn create_ticket(user: &User) -> Ticket {
    let ticket = Ticket {
        owner: (*user).clone(),
        value: random_ticket_number(),
    };
    return ticket;
}

pub fn create_user(addr: Addr) -> User {
    let user = User {
        addr: addr,
        tickets: Vec::new(),
    };
    return user;
}


