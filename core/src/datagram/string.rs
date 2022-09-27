pub fn string_trail<'s>(string: &'s mut String, limit: usize) -> &'s String
{
    if string.len() > limit
    {
        string.truncate(limit - 3);
        string.insert_str(limit - 3, "...");
    }
    string
}

pub fn string_limit<'s>(string: &'s String, limit: usize) -> Result<&'s String, StringLimitError>
{
    if string.len() > limit
    {
        return Err(StringLimitError)
    }

    Ok(string)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct StringLimitError;

impl std::fmt::Display for StringLimitError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String size is over the limit")
    }
}

#[cfg(test)]
mod test
{
    use super::StringLimitError;
    use super::string_trail;
    use super::string_limit;

    
    #[test]
    fn overlimit_string_limit()
    {
        let mut string = String::from("123456789");
        let expected = Err(StringLimitError);

        assert_eq!(string_limit(&mut string, 5), expected);
    }

    #[test]
    fn exactlimit_string_limit()
    {
        let mut string = String::from("12345");
        assert_eq!(string_limit(&mut string, 5), Ok(&String::from("12345")));
    }

    #[test]
    fn underlimit_string_limit()
    {
        let mut string = String::from("1");
        assert_eq!(string_limit(&mut string, 5), Ok(&String::from("12345")));
    }

    #[test]
    fn overlimit_string_trail()
    {
        let mut string = String::from("123456789123456789123456789123456789");
        string_trail(&mut string, 32);
        assert_eq!(string.len(), 32);
        assert_eq!("12345678912345678912345678912...", string);
    }

    #[test]
    fn exactlimit_string_trail()
    {
        let mut string = String::from("12345678912345678912345678912345");
        string_trail(&mut string, 32);
        assert_eq!(string.len(), 32);
        assert_eq!("12345678912345678912345678912345", string);
    }

    #[test]
    fn underlimit_string_trail()
    {
        let mut string = String::from("123456789");
        string_trail(&mut string, 32);
        assert_eq!(string.len(), 9);
        assert_eq!("123456789", string);
    }
}