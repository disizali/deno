// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::DocParser;
use crate::colors;
use serde_json;
use serde_json::json;

use super::parser::DocFileLoader;
use crate::op_error::OpError;
use std::collections::HashMap;

use futures::Future;
use futures::FutureExt;
use std::pin::Pin;

pub struct TestLoader {
  files: HashMap<String, String>,
}

impl TestLoader {
  pub fn new(files_vec: Vec<(String, String)>) -> Box<Self> {
    let mut files = HashMap::new();

    for file_tuple in files_vec {
      files.insert(file_tuple.0, file_tuple.1);
    }

    Box::new(Self { files })
  }
}

impl DocFileLoader for TestLoader {
  fn load_source_code(
    &self,
    specifier: &str,
  ) -> Pin<Box<dyn Future<Output = Result<String, OpError>>>> {
    eprintln!("specifier {:#?}", specifier);
    let res = match self.files.get(specifier) {
      Some(source_code) => Ok(source_code.to_string()),
      None => Err(OpError::other("not found".to_string())),
    };

    async move { res }.boxed_local()
  }
}

#[tokio::test]
async fn export_fn() {
  let source_code = r#"/**
* Hello there, this is a multiline JSdoc.
*
* It has many lines
*
* Or not that many?
*/
export function foo(a: string, b: number, cb: (...cbArgs: unknown[]) => void, ...args: unknown[]): void {
    console.log("Hello world");
}
"#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "functionDef": {
      "isAsync": false,
      "isGenerator": false,
      "typeParams": [],
      "params": [
          {
            "name": "a",
            "kind": "identifier",
            "tsType": {
              "keyword": "string",
              "kind": "keyword",
              "repr": "string",
            },
          },
          {
            "name": "b",
            "kind": "identifier",
            "tsType": {
              "keyword": "number",
              "kind": "keyword",
              "repr": "number",
            },
          },
          {
            "name": "cb",
            "kind": "identifier",
            "tsType": {
              "repr": "",
              "kind": "fnOrConstructor",
              "fnOrConstructor": {
                "constructor": false,
                "tsType": {
                  "keyword": "void",
                  "kind": "keyword",
                  "repr": "void"
                },
                "typeParams": [],
                "params": [{
                  "kind": "rest",
                  "name": "cbArgs",
                  "tsType": {
                    "repr": "",
                    "kind": "array",
                    "array": {
                        "repr": "unknown",
                        "kind": "keyword",
                        "keyword": "unknown"
                    }
                  },
                }]
              }
            },
          },
          {
            "name": "args",
            "kind": "rest",
            "tsType": {
              "repr": "",
              "kind": "array",
              "array": {
                  "repr": "unknown",
                  "kind": "keyword",
                  "keyword": "unknown"
              }
            }
          }
      ],
      "returnType": {
        "keyword": "void",
        "kind": "keyword",
        "repr": "void",
      },
    },
    "jsDoc": "Hello there, this is a multiline JSdoc.\n\nIt has many lines\n\nOr not that many?",
    "kind": "function",
    "location": {
      "col": 0,
      "filename": "test.ts",
      "line": 8,
    },
    "name": "foo",
  });

  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("Hello there")
  );
}

#[tokio::test]
async fn export_fn2() {
  let source_code = r#"
interface AssignOpts {
  a: string;
  b: number;
}

export function foo([e,,f, ...g]: number[], { c, d: asdf, i = "asdf", ...rest}, ops: AssignOpts = {}): void {
    console.log("Hello world");
}
"#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "functionDef": {
      "isAsync": false,
      "isGenerator": false,
      "typeParams": [],
      "params": [
        {
          "name": "",
          "kind": "array",
          "tsType": {
            "repr": "",
            "kind": "array",
            "array": {
                "repr": "number",
                "kind": "keyword",
                "keyword": "number"
            }
          }
        },
        {
          "name": "",
          "kind": "object",
          "tsType": null
        },
        {
          "name": "ops",
          "kind": "identifier",
          "tsType": {
            "repr": "AssignOpts",
            "kind": "typeRef",
            "typeRef": {
              "typeName": "AssignOpts",
              "typeParams": null,
            }
          }
        },
      ],
      "returnType": {
        "keyword": "void",
        "kind": "keyword",
        "repr": "void",
      },
    },
    "jsDoc": null,
    "kind": "function",
    "location": {
      "col": 0,
      "filename": "test.ts",
      "line": 7,
    },
    "name": "foo",
  });

  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("foo")
  );
}

#[tokio::test]
async fn export_const() {
  let source_code =
    "/** Something about fizzBuzz */\nexport const fizzBuzz = \"fizzBuzz\";\n";
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "variable",
    "name": "fizzBuzz",
    "location": {
      "filename": "test.ts",
      "line": 2,
      "col": 0
    },
    "jsDoc": "Something about fizzBuzz",
    "variableDef": {
      "tsType": null,
      "kind": "const"
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("Something about fizzBuzz")
  );
}

#[tokio::test]
async fn export_class() {
  let source_code = r#"
/** Class doc */
export class Foobar extends Fizz implements Buzz, Aldrin {
    private private1: boolean;
    protected protected1: number;
    public public1: boolean;
    public2: number;

    /** Constructor js doc */
    constructor(name: string, private private2: number, protected protected2: number) {}

    /** Async foo method */
    async foo(): Promise<void> {
        //
    }

    /** Sync bar method */
    bar(): void {
        //
    }
}
"#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let expected_json = json!({
    "kind": "class",
    "name": "Foobar",
    "location": {
      "filename": "test.ts",
      "line": 3,
      "col": 0
    },
    "jsDoc": "Class doc",
    "classDef": {
      "isAbstract": false,
      "superClass": "Fizz",
      "implements": ["Buzz", "Aldrin"],
      "typeParams": [],
      "constructors": [
        {
          "jsDoc": "Constructor js doc",
          "accessibility": null,
          "name": "constructor",
          "params": [
            {
              "name": "name",
              "kind": "identifier",
              "tsType": {
                "repr": "string",
                "kind": "keyword",
                "keyword": "string"
              }
            },
            {
              "name": "private2",
              "kind": "identifier",
              "tsType": {
                "repr": "number",
                "kind": "keyword",
                "keyword": "number"
              }
            },
            {
              "name": "protected2",
              "kind": "identifier",
              "tsType": {
                "repr": "number",
                "kind": "keyword",
                "keyword": "number"
              }
            }
          ],
          "location": {
            "filename": "test.ts",
            "line": 10,
            "col": 4
          }
        }
      ],
      "properties": [
        {
          "jsDoc": null,
          "tsType": {
              "repr": "boolean",
              "kind": "keyword",
              "keyword": "boolean"
          },
          "readonly": false,
          "accessibility": "private",
          "isAbstract": false,
          "isStatic": false,
          "name": "private1",
          "location": {
            "filename": "test.ts",
            "line": 4,
            "col": 4
          }
        },
        {
          "jsDoc": null,
          "tsType": {
            "repr": "number",
            "kind": "keyword",
            "keyword": "number"
          },
          "readonly": false,
          "accessibility": "protected",
          "isAbstract": false,
          "isStatic": false,
          "name": "protected1",
          "location": {
            "filename": "test.ts",
            "line": 5,
            "col": 4
          }
        },
        {
          "jsDoc": null,
          "tsType": {
            "repr": "boolean",
            "kind": "keyword",
            "keyword": "boolean"
          },
          "readonly": false,
          "accessibility": "public",
          "isAbstract": false,
          "isStatic": false,
          "name": "public1",
          "location": {
            "filename": "test.ts",
            "line": 6,
            "col": 4
          }
        },
        {
          "jsDoc": null,
          "tsType": {
            "repr": "number",
            "kind": "keyword",
            "keyword": "number"
          },
          "readonly": false,
          "accessibility": null,
          "isAbstract": false,
          "isStatic": false,
          "name": "public2",
          "location": {
            "filename": "test.ts",
            "line": 7,
            "col": 4
          }
        }
      ],
      "methods": [
        {
          "jsDoc": "Async foo method",
          "accessibility": null,
          "isAbstract": false,
          "isStatic": false,
          "name": "foo",
          "kind": "method",
          "functionDef": {
            "params": [],
            "returnType": {
                "repr": "Promise",
                "kind": "typeRef",
                "typeRef": {
                  "typeParams": [
                    {
                      "repr": "void",
                      "kind": "keyword",
                      "keyword": "void"
                    }
                  ],
                  "typeName": "Promise"
                }
            },
            "typeParams": [],
            "isAsync": true,
            "isGenerator": false
          },
          "location": {
            "filename": "test.ts",
            "line": 13,
            "col": 4
          }
        },
        {
          "jsDoc": "Sync bar method",
          "accessibility": null,
          "isAbstract": false,
          "isStatic": false,
          "name": "bar",
          "kind": "method",
          "functionDef": {
            "params": [],
            "returnType": {
              "repr": "void",
              "kind": "keyword",
              "keyword": "void"
            },
            "isAsync": false,
            "isGenerator": false,
            "typeParams": []
          },
          "location": {
            "filename": "test.ts",
            "line": 18,
            "col": 4
          }
        }
      ]
    }
  });
  let entry = &entries[0];
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("class Foobar extends Fizz implements Buzz, Aldrin")
  );
}

#[tokio::test]
async fn export_interface() {
  let source_code = r#"
/**
 * Interface js doc
 */
export interface Reader {
    /** Read n bytes */
    read(buf: Uint8Array, something: unknown): Promise<number>
}
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
      "kind": "interface",
      "name": "Reader",
      "location": {
        "filename": "test.ts",
        "line": 5,
        "col": 0
      },
      "jsDoc": "Interface js doc",
      "interfaceDef": {
        "methods": [
          {
            "name": "read",
            "location": {
              "filename": "test.ts",
              "line": 7,
              "col": 4
            },
            "jsDoc": "Read n bytes",
            "params": [
              {
                "name": "buf",
                "kind": "identifier",
                "tsType": {
                  "repr": "Uint8Array",
                  "kind": "typeRef",
                  "typeRef": {
                    "typeParams": null,
                    "typeName": "Uint8Array"
                  }
                }
              },
              {
                "name": "something",
                "kind": "identifier",
                "tsType": {
                  "repr": "unknown",
                  "kind": "keyword",
                  "keyword": "unknown"
                }
              }
            ],
            "typeParams": [],
            "returnType": {
              "repr": "Promise",
              "kind": "typeRef",
              "typeRef": {
                "typeParams": [
                  {
                    "repr": "number",
                    "kind": "keyword",
                    "keyword": "number"
                  }
                ],
                "typeName": "Promise"
              }
            }
          }
        ],
        "properties": [],
        "callSignatures": [],
        "typeParams": [],
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("interface Reader")
  );
}

#[tokio::test]
async fn export_interface2() {
  let source_code = r#"
export interface TypedIface<T> {
    something(): T
}
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
      "kind": "interface",
      "name": "TypedIface",
      "location": {
        "filename": "test.ts",
        "line": 2,
        "col": 0
      },
      "jsDoc": null,
      "interfaceDef": {
        "methods": [
          {
            "name": "something",
            "location": {
              "filename": "test.ts",
              "line": 3,
              "col": 4
            },
            "jsDoc": null,
            "params": [],
            "typeParams": [],
            "returnType": {
              "repr": "T",
              "kind": "typeRef",
              "typeRef": {
                "typeParams": null,
                "typeName": "T"
              }
            }
          }
        ],
        "properties": [],
        "callSignatures": [],
        "typeParams": [
          { "name": "T" }
        ],
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("interface TypedIface")
  );
}

#[tokio::test]
async fn export_type_alias() {
  let source_code = r#"
/** Array holding numbers */
export type NumberArray = Array<number>;
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "typeAlias",
    "name": "NumberArray",
    "location": {
        "filename": "test.ts",
      "line": 3,
      "col": 0
    },
    "jsDoc": "Array holding numbers",
    "typeAliasDef": {
      "typeParams": [],
      "tsType": {
        "repr": "Array",
        "kind": "typeRef",
        "typeRef": {
          "typeParams": [
            {
              "repr": "number",
              "kind": "keyword",
              "keyword": "number"
            }
          ],
          "typeName": "Array"
        }
      }
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("Array holding numbers")
  );
}

#[tokio::test]
async fn export_enum() {
  let source_code = r#"
/**
 * Some enum for good measure
 */
export enum Hello {
    World = "world",
    Fizz = "fizz",
    Buzz = "buzz",
}
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "enum",
    "name": "Hello",
    "location": {
      "filename": "test.ts",
      "line": 5,
      "col": 0
    },
    "jsDoc": "Some enum for good measure",
    "enumDef": {
      "members": [
        {
          "name": "World"
        },
        {
          "name": "Fizz"
        },
        {
          "name": "Buzz"
        }
      ]
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(colors::strip_ansi_codes(
    super::printer::format(entries.clone()).as_str()
  )
  .contains("Some enum for good measure"));
  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("enum Hello")
  );
}

#[tokio::test]
async fn export_namespace() {
  let source_code = r#"
/** Namespace JSdoc */
export namespace RootNs {
    export const a = "a";

    /** Nested namespace JSDoc */
    export namespace NestedNs {
      export enum Foo {
        a = 1,
        b = 2,
        c = 3,
      }
    }
}
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "namespace",
    "name": "RootNs",
    "location": {
      "filename": "test.ts",
      "line": 3,
      "col": 0
    },
    "jsDoc": "Namespace JSdoc",
    "namespaceDef": {
      "elements": [
        {
          "kind": "variable",
          "name": "a",
          "location": {
            "filename": "test.ts",
            "line": 4,
            "col": 4
          },
          "jsDoc": null,
          "variableDef": {
            "tsType": null,
            "kind": "const"
          }
        },
        {
          "kind": "namespace",
          "name": "NestedNs",
          "location": {
            "filename": "test.ts",
            "line": 7,
            "col": 4
          },
          "jsDoc": "Nested namespace JSDoc",
          "namespaceDef": {
            "elements": [
              {
                "kind": "enum",
                "name": "Foo",
                "location": {
                  "filename": "test.ts",
                  "line": 8,
                  "col": 6
                },
                "jsDoc": null,
                "enumDef": {
                  "members": [
                    {
                      "name": "a"
                    },
                    {
                      "name": "b"
                    },
                    {
                      "name": "c"
                    }
                  ]
                }
              }
            ]
          }
        }
      ]
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);
  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("namespace RootNs")
  );
}

#[tokio::test]
async fn declare_namespace() {
  let source_code = r#"
/** Namespace JSdoc */
declare namespace RootNs {
    declare const a = "a";

    /** Nested namespace JSDoc */
    declare namespace NestedNs {
      declare enum Foo {
        a = 1,
        b = 2,
        c = 3,
      }
    }
}
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "namespace",
    "name": "RootNs",
    "location": {
      "filename": "test.ts",
      "line": 3,
      "col": 0
    },
    "jsDoc": "Namespace JSdoc",
    "namespaceDef": {
      "elements": [
        {
          "kind": "variable",
          "name": "a",
          "location": {
            "filename": "test.ts",
            "line": 4,
            "col": 12
          },
          "jsDoc": null,
          "variableDef": {
            "tsType": null,
            "kind": "const"
          }
        },
        {
          "kind": "namespace",
          "name": "NestedNs",
          "location": {
            "filename": "test.ts",
            "line": 7,
            "col": 4
          },
          "jsDoc": "Nested namespace JSDoc",
          "namespaceDef": {
            "elements": [
              {
                "kind": "enum",
                "name": "Foo",
                "location": {
                  "filename": "test.ts",
                  "line": 8,
                  "col": 6
                },
                "jsDoc": null,
                "enumDef": {
                  "members": [
                    {
                      "name": "a"
                    },
                    {
                      "name": "b"
                    },
                    {
                      "name": "c"
                    }
                  ]
                }
              }
            ]
          }
        }
      ]
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);
  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("namespace RootNs")
  );
}
#[tokio::test]
async fn optional_return_type() {
  let source_code = r#"
  export function foo(a: number) {
    return a;
  }
    "#;
  let loader =
    TestLoader::new(vec![("test.ts".to_string(), source_code.to_string())]);
  let entries = DocParser::new(loader).parse("test.ts").await.unwrap();
  assert_eq!(entries.len(), 1);
  let entry = &entries[0];
  let expected_json = json!({
    "kind": "function",
    "name": "foo",
    "location": {
      "filename": "test.ts",
      "line": 2,
      "col": 2
    },
    "jsDoc": null,
    "functionDef": {
      "params": [
          {
            "name": "a",
            "kind": "identifier",
            "tsType": {
              "keyword": "number",
              "kind": "keyword",
              "repr": "number",
            },
          }
      ],
      "typeParams": [],
      "returnType": null,
      "isAsync": false,
      "isGenerator": false
    }
  });
  let actual = serde_json::to_value(entry).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("function foo(a: number)")
  );
}

#[tokio::test]
async fn reexports() {
  let nested_reexport_source_code = r#"
/**
  * JSDoc for bar
  */
export const bar = "bar";
"#;
  let reexport_source_code = r#"
import { bar } from "./nested_reexport.ts";

/**
 * JSDoc for const
 */
export const foo = "foo";
"#;
  let test_source_code = r#"
export { foo as fooConst } from "./reexport.ts";

/** JSDoc for function */
export function fooFn(a: number) {
  return a;
}
"#;
  let loader = TestLoader::new(vec![
    ("file:///test.ts".to_string(), test_source_code.to_string()),
    (
      "file:///reexport.ts".to_string(),
      reexport_source_code.to_string(),
    ),
    (
      "file:///nested_reexport.ts".to_string(),
      nested_reexport_source_code.to_string(),
    ),
  ]);
  let entries = DocParser::new(loader)
    .parse_with_reexports("file:///test.ts")
    .await
    .unwrap();
  assert_eq!(entries.len(), 2);

  let expected_json = json!([
    {
      "kind": "variable",
      "name": "fooConst",
      "location": {
        "filename": "file:///reexport.ts",
        "line": 7,
        "col": 0
      },
      "jsDoc": "JSDoc for const",
      "variableDef": {
        "tsType": null,
        "kind": "const"
      }
    },
    {
      "kind": "function",
      "name": "fooFn",
      "location": {
        "filename": "file:///test.ts",
        "line": 5,
        "col": 0
      },
      "jsDoc": "JSDoc for function",
      "functionDef": {
        "params": [
            {
              "name": "a",
              "kind": "identifier",
              "tsType": {
                "keyword": "number",
                "kind": "keyword",
                "repr": "number",
              },
            }
        ],
        "typeParams": [],
        "returnType": null,
        "isAsync": false,
        "isGenerator": false
      }
    }
  ]);
  let actual = serde_json::to_value(entries.clone()).unwrap();
  assert_eq!(actual, expected_json);

  assert!(
    colors::strip_ansi_codes(super::printer::format(entries).as_str())
      .contains("function fooFn(a: number)")
  );
}
