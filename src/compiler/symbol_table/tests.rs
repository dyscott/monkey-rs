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
        let result = global.borrow().resolve(&sym.name);
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
        let result = local.borrow().resolve(&sym.name);
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
        let result = local.borrow().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }

    for sym in expected2 {
        let result = local2.borrow().resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}