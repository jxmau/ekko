use mxg::{buffer::Buffer, message::DebugMessage, Message};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nature {
    CreateDatabase,
    CreateTable,
    Select,
    /// Error is not a SQL Nature, but a placeholder to stop, and see it there's an early return.
    Error,
}

impl Nature {

    pub fn new(command: &str, file: &str) -> Option<Self> {
        use mxg::message::Kind::Error;
        let mut it = command.rsplit(" ");
        
        
        match it.next() {
            None => {
                let m = DebugMessage::new(Error, "Invalid start of Query", 1).locate(file, 1, 1).explain("No query found.");
                None   
            },
            Some(s) => {
                match s.to_uppercase() as str {
                    "CREATE" => {
                        match s.next() {

                        }
                    }
                    _ => {
                        let m = DebugMessage::new(Error, "Invalid start of Query", 1).locate(file, 1, 1).explain("No query found.");
                        return Some(Nature::Error); // Early return   

                    }
                }
            
                Some(Nature::CreateDatabase)
            }
        }
    } 

}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn invalid_start_of_query(){
        assert_eq!(Nature::Error, Nature::new("", "test.sql"));                
    }


}