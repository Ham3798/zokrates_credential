#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::fs::File;
    use std::io;

    fn setup(path: &Path) {
        if path.exists() {
            let mut file = File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Keypair::from_bytes(&bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            let mut csprng = OsRng{};
            let keypair = Keypair::generate(&mut csprng);
    
            let bytes = keypair.to_bytes();
            let mut file = File::create(path)?;
            file.write_all(&bytes)?;
    
            Ok(keypair)
        }
    }
}
