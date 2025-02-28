pub struct Account {
    _name: String,
    _pub_key: String,
    _priv_key: String,
}

impl Account {
    // TODO: Able to write the account data to the file
    pub fn create(name: String) -> Account {
        Account {
            _name: name,
            _pub_key: String::new(),
            _priv_key: String::new(),
        }
    }

    // TODO: Able to read the account data from the file
    pub fn load(name: String) -> Account {
        Account {
            _name: name,
            _pub_key: String::new(),
            _priv_key: String::new(),
        }
    }

    // TODO: Able to sign data
}
