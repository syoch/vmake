use nom::number::complete::u8;
use nom::IResult;
pub enum Type {
    File = 0,
    Folder = 1,
    RemoteResource = 2,
}

impl Type {
    pub fn from(input: u8) -> Type {
        match input {
            0 => Type::File,
            1 => Type::Folder,
            2 => Type::RemoteResource,
            _ => panic!("Unknown type"),
        }
    }

    pub fn read(input: &[u8]) -> IResult<&[u8], Type> {
        let (input, t) = u8(input)?;

        Ok((input, Type::from(t)))
    }
}
