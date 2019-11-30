use mockall::*;
use mockall::predicate::*;

#[derive(Debug, PartialEq)]
struct FromTo<'a>(&'a str, &'a str);

#[automock]
trait Resource {
    fn get_support_lang_list(&self) -> Result<Vec<String>, TranslateError>;

    fn translate<'b>(&self, text: &String, info: &FromTo<'b>) -> Result<String, TranslateError>;
}

#[derive(Debug, PartialEq, Clone)]
enum TranslateError {
    Lang(String),
    Resource(String),
}

impl std::fmt::Display for TranslateError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Lang(ref data) => write!(f, "{}", data),
            Self::Resource(ref data) => write!(f, "{}", data),
        }
    }
} 

impl std::error::Error for TranslateError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lang_list_success() {
        let mut mock_resource = MockResource::new();

        mock_resource.expect_get_support_lang_list()
            .return_const(Ok(vec![String::from("ru"), String::from("en")]));

        assert_eq!(
            Ok(vec![String::from("ru"), String::from("en")]),
            mock_resource.get_support_lang_list()
        );
    }

    #[test]
    fn test_lang_list_error() {
        let mut mock_resource = MockResource::new();

        mock_resource.expect_get_support_lang_list()
            .return_const(Err(TranslateError::Lang("Unsoported method".to_string())));

        assert_eq!(
            true,
            mock_resource.get_support_lang_list().is_err()
        );
    }

    #[test]
    fn test_lang_translate_resource() {
        let mut mock_resource = MockResource::new();

        mock_resource.expect_translate()
            .withf(|text, from_to| text == "Hello" && &FromTo("en", "ru") == from_to)
            .return_const(Err(TranslateError::Resource("Wrong params".to_string())));

        assert_eq!(
            true,
            mock_resource.translate(&String::from("Hello"), &FromTo("en", "ru")).is_err()
        );
    }

    #[test]
    fn test_lang_translate_success() {
        let mut mock_resource = MockResource::new();

        mock_resource.expect_translate()
            .withf(|text, from_to| text == "Hello" && &FromTo("en", "ru") == from_to)
            .return_const(Ok(String::from("Привет")));

        assert_eq!(
            Ok(String::from("Привет")),
            mock_resource.translate(&String::from("Hello"), &FromTo("en", "ru"))
        );
    }
}
