#[derive(PartialEq, Debug)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Location {
    fn from(tuple: (usize, usize)) -> Self {
        Location {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Location {
    pub fn to_string(&self) -> String {
        format!("{x},{y}", x = self.x, y = self.y)
    }

    pub fn from_string(target: String) -> Result<Location, String> {
        let target = target.trim_end_matches('\n');
        let coords: Vec<&str> = target.split(',').collect();

        if coords.len() != 2 {
            return Err("Wrong number of arguments in coords".to_string());
        }

        let x: usize = match coords[0].parse() {
            Ok(x) => x,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let y: usize = match coords[1].parse() {
            Ok(y) => y,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        Ok(Location { x, y })
    }
}
