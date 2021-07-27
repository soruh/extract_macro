#[macro_export]
macro_rules! extract {
    ($src: pat, $dest: expr, $val: expr $(,)?) => {
        if let $src = $val {
            Some($dest)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! extract_fn {
    (move $src: pat, $dest: expr $(,)?) => {
        move |__input_variable_that_doesn_t_shaddow_a_different_variable__| -> Option<_> {
            $crate::extract!(
                $src,
                $dest,
                __input_variable_that_doesn_t_shaddow_a_different_variable__
            )
        }
    };
    ($src: pat, $dest: expr $(,)?) => {
        |__input_variable_that_doesn_t_shaddow_a_different_variable__| -> Option<_> {
            $crate::extract!(
                $src,
                $dest,
                __input_variable_that_doesn_t_shaddow_a_different_variable__
            )
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_enum_variant() {
        enum FooBar {
            Foo(u8),
            Bar(String),
        }

        let a = FooBar::Foo(8);
        let b = FooBar::Bar("Bar".to_owned());

        let res = extract!(FooBar::Foo(x), x.to_string(), a).or(extract!(FooBar::Bar(x), x, b));

        assert_eq!(res.as_deref(), Some("8"));
    }

    #[test]
    fn extract_nested() {
        enum Enum1 {
            A(u8),
            B(Struct1),
        }

        struct Struct1 {
            a: u8,
            b: Enum2,
            c: Enum2,
            d: String,
        }

        enum Enum2 {
            A(String),
            B(Struct2),
        }

        struct Struct2 {
            a: u8,
            b: u16,
        }

        let val = Enum1::B(Struct1 {
            a: 5,
            b: Enum2::A("baz".to_owned()),
            c: Enum2::B(Struct2 { a: 10, b: 42 }),
            d: "foo bar".to_owned(),
        });

        let extractor = extract_fn!(
            Enum1::B(Struct1 {
                a,
                b: Enum2::A(b),
                c: Enum2::B(Struct2 { a: c, b: d }),
                d: e,
            }),
            (a, b, c as u16 + d, e),
        );

        assert_eq!(
            extractor(val),
            Some((5, "baz".to_owned(), 52, "foo bar".to_owned()))
        );

        assert_eq!(extractor(Enum1::A(6)), None);
    }

    #[test]
    fn test_extract_fn_with_captured_variable() {
        let label = String::from("bar value");

        enum FooBar {
            Foo(u8),
            Bar(String),
        }

        let extractor = extract_fn!(move FooBar::Bar(x), format!("{}: {:?}", label, x));

        assert_eq!(extractor(FooBar::Foo(6)), None);
        assert_eq!(
            extractor(FooBar::Bar("lorem ipsum".to_owned())).as_deref(),
            Some("bar value: \"lorem ipsum\"")
        );
    }
}
