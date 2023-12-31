use super::*;

#[test]
fn test_define() {
    let expected = HashMap::from([
        ("a".to_string(), Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        }),
        ("b".to_string(), Symbol {
            name: "b".to_string(),
            scope: GLOBAL_SCOPE,
            index: 1,
        }),
        ("c".to_string(), Symbol {
            name: "c".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        }),
        ("d".to_string(), Symbol {
            name: "d".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        }),
        ("e".to_string(), Symbol {
            name: "e".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        }),
        ("f".to_string(), Symbol {
            name: "f".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        }),
    ]);

    let global = SymbolTable::new(None);
    
    let a = global.borrow_mut().define("a");
    assert_eq!(a, expected["a"]);

    let b = global.borrow_mut().define("b");
    assert_eq!(b, expected["b"]);

    let local1 = SymbolTable::new(Some(global));

    let c = local1.borrow_mut().define("c");
    assert_eq!(c, expected["c"]);

    let d = local1.borrow_mut().define("d");
    assert_eq!(d, expected["d"]);

    let local2 = SymbolTable::new(Some(local1));

    let e = local2.borrow_mut().define("e");
    assert_eq!(e, expected["e"]);

    let f = local2.borrow_mut().define("f");
    assert_eq!(f, expected["f"]);
}

#[test]
fn test_resolve_global() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "b".to_string(),
            scope: GLOBAL_SCOPE,
            index: 1,
        }
    ];

    for sym in expected {
        let result = global.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}

#[test]
fn test_resolve_local() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let local = SymbolTable::new(Some(global));
    local.borrow_mut().define("c");
    local.borrow_mut().define("d");

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "b".to_string(),
            scope: GLOBAL_SCOPE,
            index: 1,
        },
        Symbol {
            name: "c".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "d".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        }
    ];

    for sym in expected {
        let result = local.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}

#[test]
fn test_resolve_nested_local() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let local = SymbolTable::new(Some(global.clone()));
    local.borrow_mut().define("c");
    local.borrow_mut().define("d");

    let local2 = SymbolTable::new(Some(local.clone()));
    local2.borrow_mut().define("e");
    local2.borrow_mut().define("f");

    let expected1 = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "b".to_string(),
            scope: GLOBAL_SCOPE,
            index: 1,
        },
        Symbol {
            name: "c".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "d".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        }
    ];

    let expected2 = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "b".to_string(),
            scope: GLOBAL_SCOPE,
            index: 1,
        },
        Symbol {
            name: "e".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "f".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        }
    ];

    for sym in expected1 {
        let result = local.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }

    for sym in expected2 {
        let result = local2.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}

#[test]
fn test_define_resolve_builtins() {
    let global = SymbolTable::new(None);
    let local1 = SymbolTable::new(Some(global.clone()));
    let local2 = SymbolTable::new(Some(local1.clone()));

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: BUILTIN_SCOPE,
            index: 0,
        },
        Symbol {
            name: "c".to_string(),
            scope: BUILTIN_SCOPE,
            index: 1,
        },
        Symbol {
            name: "e".to_string(),
            scope: BUILTIN_SCOPE,
            index: 2,
        },
        Symbol {
            name: "f".to_string(),
            scope: BUILTIN_SCOPE,
            index: 3,
        },
    ];

    for (i, sym) in expected.iter().enumerate() {
        global.borrow_mut().define_builtin(i, &sym.name);
    }

    for table in vec![global, local1, local2] {
        for sym in expected.iter() {
            let result = table.borrow_mut().resolve(&sym.name);
            assert_eq!(result, Some(sym.clone()));
        }
    }
}

#[test]
fn test_resolve_free() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let local1 = SymbolTable::new(Some(global.clone()));
    local1.borrow_mut().define("c");
    local1.borrow_mut().define("d");

    let local2 = SymbolTable::new(Some(local1.clone()));
    local2.borrow_mut().define("e");
    local2.borrow_mut().define("f");

    struct Test {
        table: Rc<RefCell<SymbolTable>>,
        expected_symbols: Vec<Symbol>,
        expected_free_symbols: Vec<Symbol>,
    }

    let tests = vec![
        Test {
            table: local1.clone(),
            expected_symbols: vec![
                Symbol {
                    name: "a".to_string(),
                    scope: GLOBAL_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "b".to_string(),
                    scope: GLOBAL_SCOPE,
                    index: 1,
                },
                Symbol {
                    name: "c".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "d".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 1,
                },
            ],
            expected_free_symbols: vec![],
        },
        Test {
            table: local2.clone(),
            expected_symbols: vec![
                Symbol {
                    name: "a".to_string(),
                    scope: GLOBAL_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "b".to_string(),
                    scope: GLOBAL_SCOPE,
                    index: 1,
                },
                Symbol {
                    name: "c".to_string(),
                    scope: FREE_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "d".to_string(),
                    scope: FREE_SCOPE,
                    index: 1,
                },
                Symbol {
                    name: "e".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "f".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 1,
                },
            ],
            expected_free_symbols: vec![
                Symbol {
                    name: "c".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 0,
                },
                Symbol {
                    name: "d".to_string(),
                    scope: LOCAL_SCOPE,
                    index: 1,
                },
            ],
        },
    ];

    for test in tests {
        let table = test.table;
        let expected_symbols = test.expected_symbols;
        let expected_free_symbols = test.expected_free_symbols;

        for sym in expected_symbols {
            let result = table.borrow_mut().resolve(&sym.name);
            assert_eq!(result, Some(sym));
        }

        let free_symbols = &table.borrow_mut().free_symbols;
        assert_eq!(free_symbols.len(), expected_free_symbols.len());
        for (i, sym) in free_symbols.iter().enumerate() {
            assert_eq!(sym, &expected_free_symbols[i]);
        }
    }
}

#[test]
fn test_resolve_unresolvable_free() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define("a");

    let local1 = SymbolTable::new(Some(global.clone()));
    local1.borrow_mut().define("c");

    let local2 = SymbolTable::new(Some(local1.clone()));
    local2.borrow_mut().define("e");
    local2.borrow_mut().define("f");

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "c".to_string(),
            scope: FREE_SCOPE,
            index: 0,
        },
        Symbol {
            name: "e".to_string(),
            scope: LOCAL_SCOPE,
            index: 0,
        },
        Symbol {
            name: "f".to_string(),
            scope: LOCAL_SCOPE,
            index: 1,
        },
    ];

    for sym in expected {
        let result = local2.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }

    let expect_unresolvable = vec![
        String::from("b"),
        String::from("d"),
    ];

    for name in expect_unresolvable {
        let result = local2.borrow_mut().resolve(&name);
        assert_eq!(result, None);
    }
}

#[test]
fn test_define_and_resolve_function_name() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define_function_name("a");

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: FUNCTION_SCOPE,
            index: 0,
        },
    ];
    
    for sym in expected {
        let result = global.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}

#[test]
fn test_shadowing_function_name() {
    let global = SymbolTable::new(None);
    global.borrow_mut().define_function_name("a");
    global.borrow_mut().define("a");

    let expected = vec![
        Symbol {
            name: "a".to_string(),
            scope: GLOBAL_SCOPE,
            index: 0,
        },
    ];
    
    for sym in expected {
        let result = global.borrow_mut().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}