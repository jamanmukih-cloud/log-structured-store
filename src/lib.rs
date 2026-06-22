pub struct Store {
    path: String,
}

impl Store {
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { path: path.to_string() })
    }
    
    pub fn put(&self, _key: &[u8], _value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    pub fn get(&self, _key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    pub fn delete(&self, _key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
