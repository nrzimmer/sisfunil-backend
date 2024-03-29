use regex::Regex;
use serde::Deserialize;

#[macro_export]
macro_rules! gen_filter_fn {
    ($func_name:tt, $filter_field_ty:ty, $filter_field_expr:expr) => {
        fn $func_name<T: 'static>(
            filter: Filter,
        ) -> Box<dyn BoxableExpression<T, Mysql, SqlType = Bool>>
        where
            diesel::dsl::Like<$filter_field_ty, &'static str>:
                BoxableExpression<T, Mysql, SqlType = Bool>,
            $filter_field_ty: SelectableExpression<T>,
        {
            let mut words = filter.words.clone();
            let first = Box::new($filter_field_expr.like(format!("%{}%", words.pop().unwrap())));
            let result: Box<dyn BoxableExpression<T, Mysql, SqlType = Bool>> = words
                .into_iter()
                .map(|word| $filter_field_expr.like(format!("%{}%", word)))
                .fold(first, |predicate, next_predicate| {
                    Box::new(predicate.and(next_predicate))
                });
            result
        }
    };
}

#[macro_export]
macro_rules! gen_filters_fn {
    ($func_name:tt, $filter_field_1_ty:ty, $filter_field_1_expr:expr, $filter_field_2_ty:ty, $filter_field_2_expr:expr) => {
        fn $func_name<T: 'static>(
            filter: Filter,
        ) -> Box<dyn BoxableExpression<T, Mysql, SqlType = Bool>>
        where
            diesel::dsl::Like<$filter_field_1_ty, &'static str>:
                BoxableExpression<T, Mysql, SqlType = Bool>,
            diesel::dsl::Like<$filter_field_2_ty, &'static str>:
                BoxableExpression<T, Mysql, SqlType = Bool>,
            $filter_field_1_ty: SelectableExpression<T>,
            $filter_field_2_ty: SelectableExpression<T>,
        {
            let mut words = filter.words.clone();
            let text = format!("%{}%", words.pop().unwrap());
            let first = Box::new(
                $filter_field_1_expr
                    .like(text.clone())
                    .or($filter_field_2_expr.like(text)),
            );
            let result: Box<dyn BoxableExpression<T, Mysql, SqlType = Bool>> = words
                .into_iter()
                .map(|word| {
                    let text = format!("%{}%", word);
                    $filter_field_1_expr
                        .like(text.clone())
                        .or($filter_field_2_expr.like(text))
                })
                .fold(first, |predicate, next_predicate| {
                    Box::new(predicate.and(next_predicate))
                });
            result
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct FilterConfig {
    pub filter: String,
}

pub trait ToFilter {
    fn to_filter(&self) -> Filter;
}

#[derive(Clone)]
pub struct Filter {
    pub words: Vec<String>,
}

impl ToFilter for FilterConfig {
    fn to_filter(&self) -> Filter {
        let re = Regex::new(r#""([^"]*)"|\b(\w+)\b"#).unwrap();

        let mut captured_words: Vec<String> = Vec::new();

        for capture in re.captures_iter(self.filter.as_str()) {
            if let Some(quoted_word) = capture.get(1) {
                captured_words.push(quoted_word.as_str().to_string());
            } else if let Some(word) = capture.get(2) {
                captured_words.push(word.as_str().to_string());
            }
        }

        Filter {
            words: captured_words.clone(),
        }
    }
}
