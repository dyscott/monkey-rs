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
    ]);

    let mut global = SymbolTable::new();
    
    let a = global.define("a");
    assert_eq!(a, expected["a"]);

    let b = global.define("b");
    assert_eq!(b, expected["b"]);
}

#[test]
fn test_resolve_global() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

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
        let result = global.resolve(&sym.name);
        assert_eq!(result, Some(sym));
    }
}