#[macro_export]
macro_rules! extract {
    ($src: pat $(if $guard: expr)?, $dest: expr, $val: expr $(,)?) => {
        match $val {
            $src $(if $guard)? => ::core::option::Option::Some($dest),
            _ => ::core::option::Option::None,
        }
    };
}

#[macro_export]
macro_rules! extract_fn {
    (move, $src: pat $(if $guard: expr)?, $dest: expr $(,)?) => {
        move |__input_variable_that_doesn_t_shaddow_a_different_variable__| -> ::core::option::Option<_> {
            $crate::extract!(
                $src $(if $guard)?,
                $dest,
                __input_variable_that_doesn_t_shaddow_a_different_variable__
            )
        }
    };
    ($src: pat $(if $guard: expr)?, $dest: expr $(,)?) => {
        |__input_variable_that_doesn_t_shaddow_a_different_variable__| -> ::core::option::Option<_> {
            $crate::extract!(
                $src $(if $guard)?,
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
        enum FooBar {
            Foo(u8),
            Bar(String),
        }

        // This functions is only here so that we require the `move`.
        fn make_extractor() -> impl FnMut(FooBar) -> Option<String> {
            let mut i = 0;
            extract_fn!(
                move,
                FooBar::Bar(x),
                format!(
                    "bar value[{}]: {:?}",
                    {
                        i += 1;
                        i - 1
                    },
                    x
                )
            )
        }

        let mut extractor = make_extractor();

        assert_eq!(extractor(FooBar::Foo(6)), None);
        assert_eq!(
            extractor(FooBar::Bar("lorem ipsum".to_owned())).as_deref(),
            Some("bar value[0]: \"lorem ipsum\"")
        );
        assert_eq!(extractor(FooBar::Foo(10)), None);
        assert_eq!(
            extractor(FooBar::Bar("foo bar".to_owned())).as_deref(),
            Some("bar value[1]: \"foo bar\"")
        );
        assert_eq!(
            extractor(FooBar::Bar("baz".to_owned())).as_deref(),
            Some("bar value[2]: \"baz\"")
        );
    }

    #[test]
    fn the_downside_of_extractor_fn() {
        let extractor = extract_fn!((a, b, _) if a < 5, (a, b));

        assert_eq!(extractor((1, 2, 3)), Some((1, 2)));
        assert_eq!(extractor((10, 2, 3)), None);

        // This doesn't compile
        // assert_eq!(extractor((1, 2, "foo")), Some((1, 2)));
    }
}
