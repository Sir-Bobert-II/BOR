// use std::process::exit;

#[derive(Clone, Default, PartialEq)]
pub struct Error {
    error: String,
    code: u8,
    fatal: bool,
}

impl Error {
    pub fn new(error: String, code: u8) -> Self {
        Self { error, code, fatal: false }
    }
    
    pub fn fatal(mut self) -> Self
    {
        self.fatal = true;
        self
    }

    // pub fn report(self)
    // {
    //     eprintln!("Error: {}.", self.error);
    //     if self.fatal
    //     {
    //         exit(self.code.into());
    //     }
    // }

    // pub fn handle(res: Result<(), Error>)
    // {
    //     if let Err(err) = res
    //     {
    //         err.report()
    //     }
    // }
}

impl ToString for Error
{
    fn to_string(&self) -> String {
        format!("Error {}.", self.error)
    }
}

impl std::fmt::Debug for Error
{
    fn fmt(&self, f :&mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", self.error)
    }
}