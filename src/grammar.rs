// auto-generated: "lalrpop 0.15.2"
// sha256: 6caa69d5eb035d5de4542bc15acd171ae5fa027722b9692c4b6ae9834a0ce
use ast::{File, Item, Span, Ident, FunctionDecl};
use tokens::Token;
use codespan::ByteIndex;
use lalrpop_util::ParseError;
use void::Void;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__File {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{File, Item, Span, Ident, FunctionDecl};
    use tokens::Token;
    use codespan::ByteIndex;
    use lalrpop_util::ParseError;
    use void::Void;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(ByteIndex),
        Variant2(FunctionDecl),
        Variant3(File),
        Variant4(Ident),
        Variant5(Item),
        Variant6(::std::vec::Vec<Item>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 6, 0,
        // State 1
        0, 0, -7, 0,
        // State 2
        0, 0, 0, 0,
        // State 3
        0, 0, -10, 0,
        // State 4
        0, 0, 6, 0,
        // State 5
        0, 0, 0, 9,
        // State 6
        0, 0, -11, 0,
        // State 7
        10, 0, 0, 0,
        // State 8
        -6, 0, 0, 0,
        // State 9
        0, 11, 0, 0,
        // State 10
        0, 0, -3, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -4,
        // State 1
        -7,
        // State 2
        -12,
        // State 3
        -10,
        // State 4
        -5,
        // State 5
        0,
        // State 6
        -11,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -3,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 3, 0, 4, 0, 5, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 2, 0, 0, 7, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""extern""###,
            r###""ident""###,
        ];
        __ACTION[(__state * 4)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct FileParser {
        _priv: (),
    }

    impl FileParser {
        pub fn new() -> FileParser {
            FileParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=ParseError<ByteIndex, Token<'input>, Void>>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<File, __lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Extern if true => 2,
                    Token::Identifier(_) if true => 3,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 4 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Extern => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<File,__lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                // __File = File => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionDecl, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Item, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Item>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(9);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action9::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(6);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action6::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Extern = "extern", Ident, "(", ")" => ActionFn(15);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (4, __symbol, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Item+ => ActionFn(19);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = Extern => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 5)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* =  => ActionFn(7);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action7::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 6)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* = Item+ => ActionFn(8);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item+, Item => ActionFn(11);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Item = Item => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 9)
    }
}
pub use self::__parse__File::FileParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Item {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{File, Item, Span, Ident, FunctionDecl};
    use tokens::Token;
    use codespan::ByteIndex;
    use lalrpop_util::ParseError;
    use void::Void;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(ByteIndex),
        Variant2(FunctionDecl),
        Variant3(File),
        Variant4(Ident),
        Variant5(Item),
        Variant6(::std::vec::Vec<Item>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 4, 0,
        // State 1
        0, 0, 0, 0,
        // State 2
        0, 0, 0, 0,
        // State 3
        0, 0, 0, 6,
        // State 4
        7, 0, 0, 0,
        // State 5
        -6, 0, 0, 0,
        // State 6
        0, 8, 0, 0,
        // State 7
        0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -7,
        // State 2
        -13,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -3,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 0, 0, 3, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""extern""###,
            r###""ident""###,
        ];
        __ACTION[(__state * 4)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ItemParser {
        _priv: (),
    }

    impl ItemParser {
        pub fn new() -> ItemParser {
            ItemParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=ParseError<ByteIndex, Token<'input>, Void>>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Item, __lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Extern if true => 2,
                    Token::Identifier(_) if true => 3,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 4 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Extern => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Item,__lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                // __Item = Item => ActionFn(1);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionDecl, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Item, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Item>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(9);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action9::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(6);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action6::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Extern = "extern", Ident, "(", ")" => ActionFn(15);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (4, __symbol, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Item+ => ActionFn(19);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = Extern => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 5)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* =  => ActionFn(7);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action7::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 6)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* = Item+ => ActionFn(8);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item+, Item => ActionFn(11);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 8)
    }
}
pub use self::__parse__Item::ItemParser;

fn __action0<
    'input,
>(
    (_, __0, _): (ByteIndex, File, ByteIndex),
) -> File
{
    (__0)
}

fn __action1<
    'input,
>(
    (_, __0, _): (ByteIndex, Item, ByteIndex),
) -> Item
{
    (__0)
}

fn __action2<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, items, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    File::new(items, Span::new(l, r))
}

fn __action3<
    'input,
>(
    (_, __0, _): (ByteIndex, FunctionDecl, ByteIndex),
) -> Item
{
    Item::Extern(__0)
}

fn __action4<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, name, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    FunctionDecl::new(name, Vec::new(), Span::new(l, r))
}

fn __action5<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, id, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    Ident::new(id.as_ident().unwrap(), Span::new(l, r))
}

fn __action6<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookbehind.clone()
}

fn __action7<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::vec::Vec<Item>
{
    vec![]
}

fn __action8<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    v
}

fn __action9<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookahead.clone()
}

fn __action10<
    'input,
>(
    (_, __0, _): (ByteIndex, Item, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    vec![__0]
}

fn __action11<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    (_, e, _): (ByteIndex, Item, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    { let mut v = v; v.push(e); v }
}

fn __action12<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Ident, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
    __4: (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action9(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action13<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action9(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
        __0,
        __1,
    )
}

fn __action14<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action9(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __temp0,
        __0,
        __1,
    )
}

fn __action15<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Ident, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action16<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> File
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
    )
}

fn __action17<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Ident
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        __0,
        __temp0,
    )
}

fn __action18<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> File
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action7(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        __temp0,
    )
}

fn __action19<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action8(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (ByteIndex, Token<'input>, ByteIndex) {
    type Error = ParseError<ByteIndex, Token<'input>, Void>;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),ParseError<ByteIndex, Token<'input>, Void>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(ByteIndex, Token<'input>, ByteIndex),ParseError<ByteIndex, Token<'input>, Void>> {
    type Error = ParseError<ByteIndex, Token<'input>, Void>;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),ParseError<ByteIndex, Token<'input>, Void>> {
        value
    }
}
