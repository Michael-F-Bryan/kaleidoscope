// auto-generated: "lalrpop 0.15.2"
// sha256: a7423afe8337d4e811fcef9720797df18e18f1193d563be6029fabdde87796
use ast::{File, Item, Span, Ident, FunctionDecl, Function, FunctionCall, Expr, Literal};
use tokens::Token;
use codespan::ByteIndex;
use lalrpop_util::ParseError;
use void::Void;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{File, Item, Span, Ident, FunctionDecl, Function, FunctionCall, Expr, Literal};
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
        Variant1(Expr),
        Variant2(::std::vec::Vec<Expr>),
        Variant3(ByteIndex),
        Variant4(Vec<Expr>),
        Variant5(::std::option::Option<Expr>),
        Variant6(FunctionDecl),
        Variant7(File),
        Variant8(FunctionCall),
        Variant9(Function),
        Variant10(Ident),
        Variant11(::std::vec::Vec<Ident>),
        Variant12(Item),
        Variant13(::std::vec::Vec<Item>),
        Variant14(Literal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 6, 7,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, -14, -14, 0, 0, 0, 0,
        // State 3
        8, -12, -12, 0, 0, 0, 0,
        // State 4
        0, -13, -13, 0, 0, 0, 0,
        // State 5
        -24, -24, -24, 0, 0, 0, 0,
        // State 6
        0, -35, -35, 0, 0, 0, 0,
        // State 7
        0, -9, 0, 0, 0, 6, 7,
        // State 8
        0, -11, 0, 0, 0, 6, 7,
        // State 9
        0, 13, 0, 0, 0, 0, 0,
        // State 10
        0, -8, 14, 0, 0, 0, 0,
        // State 11
        0, -10, 15, 0, 0, 0, 0,
        // State 12
        0, -20, -20, 0, 0, 0, 0,
        // State 13
        0, -4, 0, 0, 0, -4, -4,
        // State 14
        0, -5, 0, 0, 0, -5, -5,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -36,
        // State 2
        -14,
        // State 3
        -12,
        // State 4
        -13,
        // State 5
        -24,
        // State 6
        -35,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -20,
        // State 13
        0,
        // State 14
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 9, 0, 0, 10, 11, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""def""###,
            r###""extern""###,
            r###""ident""###,
            r###""literal""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
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
        ) -> Result<Expr, __lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>
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
                    Token::Comma if true => 2,
                    Token::Def if true => 3,
                    Token::Extern if true => 4,
                    Token::Identifier(_) if true => 5,
                    Token::Number(_) if true => 6,
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
                    let __action = __ACTION[__state * 7 + __integer];
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
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Def => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Extern => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Number(_) => __Symbol::Variant0((__tok)),
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
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<ByteIndex, Token<'input>, ParseError<ByteIndex, Token<'input>, Void>>>>
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
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                // __Expr = Expr => ActionFn(2);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 23 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Function, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionDecl, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Item, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
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
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Item>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
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
        // (<Expr> ",") = Expr, "," => ActionFn(30);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
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
        // (<Expr> ",")* =  => ActionFn(28);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action28::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
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
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
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
        // (<Expr> ",")+ = Expr, "," => ActionFn(33);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 2)
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
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(34);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 2)
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
        // @L =  => ActionFn(21);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action21::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
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
        // @R =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 4)
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
        // Comma<Expr> = Expr => ActionFn(51);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Comma<Expr> =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(53);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(54);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Expr = Ident => ActionFn(9);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
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
        // Expr = Literal => ActionFn(10);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = FunctionCall => ActionFn(11);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(26);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(27);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action27::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Extern = "extern", FunctionDecl => ActionFn(44);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File =  => ActionFn(57);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action57::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Item+ => ActionFn(58);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(46);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action46::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (4, __symbol, 10)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", ")" => ActionFn(55);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", Ident+, ")" => ActionFn(56);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action56::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDef = "def", FunctionDecl, Expr => ActionFn(48);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* =  => ActionFn(16);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action16::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (0, __symbol, 14)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* = Ident+ => ActionFn(17);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident => ActionFn(24);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident+, Ident => ActionFn(25);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = Extern => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = FunctionDef => ActionFn(5);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* =  => ActionFn(19);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action19::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 17)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* = Item+ => ActionFn(20);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item => ActionFn(22);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item+, Item => ActionFn(23);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 18)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "literal" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce37<
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
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce38<
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
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 22)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__File {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{File, Item, Span, Ident, FunctionDecl, Function, FunctionCall, Expr, Literal};
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
        Variant1(Expr),
        Variant2(::std::vec::Vec<Expr>),
        Variant3(ByteIndex),
        Variant4(Vec<Expr>),
        Variant5(::std::option::Option<Expr>),
        Variant6(FunctionDecl),
        Variant7(File),
        Variant8(FunctionCall),
        Variant9(Function),
        Variant10(Ident),
        Variant11(::std::vec::Vec<Ident>),
        Variant12(Item),
        Variant13(::std::vec::Vec<Item>),
        Variant14(Literal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 7, 8, 0, 0,
        // State 1
        0, 0, 0, -29, -29, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -30, -30, 0, 0,
        // State 4
        0, 0, 0, -33, -33, 0, 0,
        // State 5
        0, 0, 0, 7, 8, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 12, 0,
        // State 7
        0, 0, 0, 0, 0, 12, 0,
        // State 8
        0, 0, 0, -34, -34, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 12, 18,
        // State 10
        19, 0, 0, 0, 0, 0, 0,
        // State 11
        -24, -24, -24, -24, -24, -24, 0,
        // State 12
        0, 0, 0, -17, -17, 0, 0,
        // State 13
        0, 0, 0, -23, -23, 0, 0,
        // State 14
        0, -14, -14, -14, -14, 0, 0,
        // State 15
        20, -12, -12, -12, -12, 0, 0,
        // State 16
        0, -13, -13, -13, -13, 0, 0,
        // State 17
        0, -35, -35, -35, -35, 0, 0,
        // State 18
        0, 23, 0, 0, 0, 12, 0,
        // State 19
        0, -9, 0, 0, 0, 12, 18,
        // State 20
        0, -27, 0, 0, 0, -27, 0,
        // State 21
        0, 28, 0, 0, 0, 12, 0,
        // State 22
        0, 0, 0, -21, -21, -21, -21,
        // State 23
        0, -11, 0, 0, 0, 12, 18,
        // State 24
        0, 30, 0, 0, 0, 0, 0,
        // State 25
        0, -8, 31, 0, 0, 0, 0,
        // State 26
        0, -28, 0, 0, 0, -28, 0,
        // State 27
        0, 0, 0, -22, -22, -22, -22,
        // State 28
        0, -10, 32, 0, 0, 0, 0,
        // State 29
        0, -20, -20, -20, -20, 0, 0,
        // State 30
        0, -4, 0, 0, 0, -4, -4,
        // State 31
        0, -5, 0, 0, 0, -5, -5,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -18,
        // State 1
        -29,
        // State 2
        -37,
        // State 3
        -30,
        // State 4
        -33,
        // State 5
        -19,
        // State 6
        0,
        // State 7
        0,
        // State 8
        -34,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -24,
        // State 12
        -17,
        // State 13
        -23,
        // State 14
        -14,
        // State 15
        -12,
        // State 16
        -13,
        // State 17
        -35,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -21,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -22,
        // State 28
        0,
        // State 29
        -20,
        // State 30
        0,
        // State 31
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 4, 0, 0, 0, 5, 0, 6, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 24, 0, 0, 25, 26, 0, 0, 0, 15, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 15, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""def""###,
            r###""extern""###,
            r###""ident""###,
            r###""literal""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                    Token::Comma if true => 2,
                    Token::Def if true => 3,
                    Token::Extern if true => 4,
                    Token::Identifier(_) if true => 5,
                    Token::Number(_) if true => 6,
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
                    let __action = __ACTION[__state * 7 + __integer];
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
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Def => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Extern => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Number(_) => __Symbol::Variant0((__tok)),
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
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                // __File = File => ActionFn(0);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 23 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Function, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionDecl, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Item, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
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
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Item>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
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
        // (<Expr> ",") = Expr, "," => ActionFn(30);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
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
        // (<Expr> ",")* =  => ActionFn(28);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action28::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
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
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
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
        // (<Expr> ",")+ = Expr, "," => ActionFn(33);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 2)
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
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(34);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 2)
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
        // @L =  => ActionFn(21);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action21::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
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
        // @R =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 4)
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
        // Comma<Expr> = Expr => ActionFn(51);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Comma<Expr> =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(53);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(54);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Expr = Ident => ActionFn(9);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
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
        // Expr = Literal => ActionFn(10);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = FunctionCall => ActionFn(11);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(26);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(27);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action27::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Extern = "extern", FunctionDecl => ActionFn(44);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File =  => ActionFn(57);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action57::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Item+ => ActionFn(58);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(46);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action46::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (4, __symbol, 10)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", ")" => ActionFn(55);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", Ident+, ")" => ActionFn(56);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action56::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDef = "def", FunctionDecl, Expr => ActionFn(48);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* =  => ActionFn(16);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action16::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (0, __symbol, 14)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* = Ident+ => ActionFn(17);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident => ActionFn(24);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident+, Ident => ActionFn(25);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = Extern => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = FunctionDef => ActionFn(5);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* =  => ActionFn(19);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action19::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 17)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* = Item+ => ActionFn(20);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item => ActionFn(22);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item+, Item => ActionFn(23);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 18)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "literal" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce38<
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
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 22)
    }
}
pub use self::__parse__File::FileParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Item {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ast::{File, Item, Span, Ident, FunctionDecl, Function, FunctionCall, Expr, Literal};
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
        Variant1(Expr),
        Variant2(::std::vec::Vec<Expr>),
        Variant3(ByteIndex),
        Variant4(Vec<Expr>),
        Variant5(::std::option::Option<Expr>),
        Variant6(FunctionDecl),
        Variant7(File),
        Variant8(FunctionCall),
        Variant9(Function),
        Variant10(Ident),
        Variant11(::std::vec::Vec<Ident>),
        Variant12(Item),
        Variant13(::std::vec::Vec<Item>),
        Variant14(Literal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 5, 6, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 9, 0,
        // State 5
        0, 0, 0, 0, 0, 9, 0,
        // State 6
        0, 0, 0, 0, 0, 9, 15,
        // State 7
        16, 0, 0, 0, 0, 0, 0,
        // State 8
        -24, -24, -24, 0, 0, -24, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -14, -14, 0, 0, 0, 0,
        // State 12
        17, -12, -12, 0, 0, 0, 0,
        // State 13
        0, -13, -13, 0, 0, 0, 0,
        // State 14
        0, -35, -35, 0, 0, 0, 0,
        // State 15
        0, 20, 0, 0, 0, 9, 0,
        // State 16
        0, -9, 0, 0, 0, 9, 15,
        // State 17
        0, -27, 0, 0, 0, -27, 0,
        // State 18
        0, 25, 0, 0, 0, 9, 0,
        // State 19
        0, 0, 0, 0, 0, -21, -21,
        // State 20
        0, -11, 0, 0, 0, 9, 15,
        // State 21
        0, 27, 0, 0, 0, 0, 0,
        // State 22
        0, -8, 28, 0, 0, 0, 0,
        // State 23
        0, -28, 0, 0, 0, -28, 0,
        // State 24
        0, 0, 0, 0, 0, -22, -22,
        // State 25
        0, -10, 29, 0, 0, 0, 0,
        // State 26
        0, -20, -20, 0, 0, 0, 0,
        // State 27
        0, -4, 0, 0, 0, -4, -4,
        // State 28
        0, -5, 0, 0, 0, -5, -5,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -29,
        // State 2
        -30,
        // State 3
        -38,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        -24,
        // State 9
        -17,
        // State 10
        -23,
        // State 11
        -14,
        // State 12
        -12,
        // State 13
        -13,
        // State 14
        -35,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -21,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        -22,
        // State 25
        0,
        // State 26
        -20,
        // State 27
        0,
        // State 28
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 13, 0, 0, 0, 0, 0, 14, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 21, 0, 0, 22, 23, 0, 0, 0, 12, 0, 0, 13, 0, 0, 0, 0, 0, 14, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 12, 0, 0, 13, 0, 0, 0, 0, 0, 14, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""def""###,
            r###""extern""###,
            r###""ident""###,
            r###""literal""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                    Token::Comma if true => 2,
                    Token::Def if true => 3,
                    Token::Extern if true => 4,
                    Token::Identifier(_) if true => 5,
                    Token::Number(_) if true => 6,
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
                    let __action = __ACTION[__state * 7 + __integer];
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
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Def => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Extern => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Number(_) => __Symbol::Variant0((__tok)),
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
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                // __Item = Item => ActionFn(1);
                let __sym0 = __pop_Variant12(__symbols);
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
        let __next_state = __GOTO[__state * 23 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Function, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionDecl, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Item, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
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
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Item>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
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
        // (<Expr> ",") = Expr, "," => ActionFn(30);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
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
        // (<Expr> ",")* =  => ActionFn(28);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action28::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
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
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
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
        // (<Expr> ",")+ = Expr, "," => ActionFn(33);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 2)
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
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(34);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 2)
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
        // @L =  => ActionFn(21);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action21::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
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
        // @R =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 4)
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
        // Comma<Expr> = Expr => ActionFn(51);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Comma<Expr> =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(53);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 5)
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
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(54);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 5)
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
        // Expr = Ident => ActionFn(9);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
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
        // Expr = Literal => ActionFn(10);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = FunctionCall => ActionFn(11);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(26);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(27);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action27::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Extern = "extern", FunctionDecl => ActionFn(44);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File =  => ActionFn(57);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action57::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Item+ => ActionFn(58);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(46);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action46::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (4, __symbol, 10)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", ")" => ActionFn(55);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDecl = Ident, "(", Ident+, ")" => ActionFn(56);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action56::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionDef = "def", FunctionDecl, Expr => ActionFn(48);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* =  => ActionFn(16);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action16::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (0, __symbol, 14)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident* = Ident+ => ActionFn(17);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident => ActionFn(24);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident+ = Ident+, Ident => ActionFn(25);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = Extern => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item = FunctionDef => ActionFn(5);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* =  => ActionFn(19);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action19::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 17)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item* = Item+ => ActionFn(20);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item => ActionFn(22);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Item+ = Item+, Item => ActionFn(23);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 18)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "literal" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce37<
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
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 21)
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
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    (__0)
}

fn __action3<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, items, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    File::new(items, Span::new(l, r))
}

fn __action4<
    'input,
>(
    (_, __0, _): (ByteIndex, FunctionDecl, ByteIndex),
) -> Item
{
    Item::Extern(__0)
}

fn __action5<
    'input,
>(
    (_, __0, _): (ByteIndex, Function, ByteIndex),
) -> Item
{
    Item::Function(__0)
}

fn __action6<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, decl, _): (ByteIndex, FunctionDecl, ByteIndex),
    (_, body, _): (ByteIndex, Expr, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Function
{
    Function::new(decl, body, Span::new(l, r))
}

fn __action7<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, decl, _): (ByteIndex, FunctionDecl, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    FunctionDecl { span: Span::new(l, r), ..decl }
}

fn __action8<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, name, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, args, _): (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    FunctionDecl::new(name, args, Span::new(l, r))
}

fn __action9<
    'input,
>(
    (_, __0, _): (ByteIndex, Ident, ByteIndex),
) -> Expr
{
    Expr::Ident(__0)
}

fn __action10<
    'input,
>(
    (_, __0, _): (ByteIndex, Literal, ByteIndex),
) -> Expr
{
    Expr::Literal(__0)
}

fn __action11<
    'input,
>(
    (_, __0, _): (ByteIndex, FunctionCall, ByteIndex),
) -> Expr
{
    Expr::FunctionCall(__0)
}

fn __action12<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, lit, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    Literal::new(lit.as_number().unwrap(), Span::new(l, r))
}

fn __action13<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, id, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    Ident::new(id.as_ident().unwrap(), Span::new(l, r))
}

fn __action14<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, name, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, args, _): (ByteIndex, Vec<Expr>, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionCall
{
    FunctionCall::new(name, args, Span::new(l, r))
}

fn __action15<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    (_, e, _): (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    match e { 
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

fn __action16<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::vec::Vec<Ident>
{
    vec![]
}

fn __action17<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
) -> ::std::vec::Vec<Ident>
{
    v
}

fn __action18<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookbehind.clone()
}

fn __action19<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::vec::Vec<Item>
{
    vec![]
}

fn __action20<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    v
}

fn __action21<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookahead.clone()
}

fn __action22<
    'input,
>(
    (_, __0, _): (ByteIndex, Item, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    vec![__0]
}

fn __action23<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    (_, e, _): (ByteIndex, Item, ByteIndex),
) -> ::std::vec::Vec<Item>
{
    { let mut v = v; v.push(e); v }
}

fn __action24<
    'input,
>(
    (_, __0, _): (ByteIndex, Ident, ByteIndex),
) -> ::std::vec::Vec<Ident>
{
    vec![__0]
}

fn __action25<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
    (_, e, _): (ByteIndex, Ident, ByteIndex),
) -> ::std::vec::Vec<Ident>
{
    { let mut v = v; v.push(e); v }
}

fn __action26<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::option::Option<Expr>
{
    Some(__0)
}

fn __action27<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::option::Option<Expr>
{
    None
}

fn __action28<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

fn __action29<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    v
}

fn __action30<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
) -> Expr
{
    (__0)
}

fn __action31<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

fn __action32<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    (_, e, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

fn __action33<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action30(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __temp0,
    )
}

fn __action34<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, Expr, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __0,
        __temp0,
    )
}

fn __action35<
    'input,
>(
    __0: (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action28(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        __temp0,
        __0,
    )
}

fn __action36<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        __temp0,
        __1,
    )
}

fn __action37<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, FunctionDecl, ByteIndex),
    __2: (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action38<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __temp0,
        __0,
        __1,
    )
}

fn __action39<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Vec<Expr>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
    __4: (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionCall
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action40<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
    __4: (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action41<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, FunctionDecl, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
    __3: (ByteIndex, ByteIndex, ByteIndex),
) -> Function
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action42<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __temp0,
        __0,
        __1,
    )
}

fn __action43<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __temp0,
        __0,
        __1,
    )
}

fn __action44<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, FunctionDecl, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __0,
        __1,
        __temp0,
    )
}

fn __action45<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> File
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        __0,
        __temp0,
    )
}

fn __action46<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Vec<Expr>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionCall
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action47<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action48<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, FunctionDecl, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
) -> Function
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action49<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Ident
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        __0,
        __temp0,
    )
}

fn __action50<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Literal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        __0,
        __temp0,
    )
}

fn __action51<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
    )
}

fn __action52<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> Vec<Expr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action27(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
    )
}

fn __action53<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, Expr, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __0,
        __temp0,
    )
}

fn __action54<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __0,
        __temp0,
    )
}

fn __action55<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action16(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        __0,
        __1,
        __temp0,
        __2,
    )
}

fn __action56<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, ::std::vec::Vec<Ident>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionDecl
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action17(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        __0,
        __1,
        __temp0,
        __3,
    )
}

fn __action57<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> File
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action19(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
    )
}

fn __action58<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Item>, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
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
