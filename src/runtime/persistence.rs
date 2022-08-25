use super::Step;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

const FILENAME: &str = "mrt.json";

pub fn delete() -> Result<(), Box<dyn Error>> {
    std::fs::remove_file(FILENAME)?;
    Ok(())
}

/// loads an Executor instance from the persistence file on disk
pub fn load() -> Option<Vec<Step>> {
    let file = match File::open(FILENAME) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let reader = BufReader::new(file);
    let result: Vec<Step> =
        serde_json::from_reader(reader).expect("cannot deserialize persisted steps");
    Some(result)
}

/// stores this Executor into the persistence file on disk
pub fn save(steps: &Vec<Step>) -> Result<(), Box<dyn Error>> {
    let file = File::create(FILENAME)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, steps)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    mod persistence {
        use crate::runtime::{load, save, Step};

        #[test]
        fn persistence() {
            let steps1 = vec![Step {
                id: 3,
                command: "git".into(),
                args: vec!["clone".into()],
            }];
            let _ = save(&steps1);
            let steps2 = load().unwrap();
            assert_eq!(steps1, steps2);
        }
    }
}
