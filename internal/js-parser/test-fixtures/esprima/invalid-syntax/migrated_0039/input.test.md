# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0039`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "esprima/invalid-syntax/migrated_0039/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/invalid-syntax/migrated_0039/input.js"
		end: Object {
			column: 6
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Unterminated regular expression"}]}
			}
			location: Object {
				filename: "esprima/invalid-syntax/migrated_0039/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 5
					line: 1
				}
				start: Object {
					column: 5
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "esprima/invalid-syntax/migrated_0039/input.js"
				end: Object {
					column: 6
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSRegExpLiteral {
				global: false
				insensitive: false
				multiline: false
				noDotNewline: false
				sticky: false
				unicode: false
				loc: Object {
					filename: "esprima/invalid-syntax/migrated_0039/input.js"
					end: Object {
						column: 6
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				expression: JSRegExpSubExpression {
					loc: Object {
						filename: "esprima/invalid-syntax/migrated_0039/input.js"
						end: Object {
							column: 5
							line: 1
						}
						start: Object {
							column: 1
							line: 1
						}
					}
					body: Array [
						JSRegExpCharacter {
							value: "t"
							loc: Object {
								filename: "esprima/invalid-syntax/migrated_0039/input.js"
								end: Object {
									column: 2
									line: 1
								}
								start: Object {
									column: 1
									line: 1
								}
							}
						}
						JSRegExpCharacter {
							value: "e"
							loc: Object {
								filename: "esprima/invalid-syntax/migrated_0039/input.js"
								end: Object {
									column: 3
									line: 1
								}
								start: Object {
									column: 2
									line: 1
								}
							}
						}
						JSRegExpCharacter {
							value: "s"
							loc: Object {
								filename: "esprima/invalid-syntax/migrated_0039/input.js"
								end: Object {
									column: 4
									line: 1
								}
								start: Object {
									column: 3
									line: 1
								}
							}
						}
						JSRegExpCharacter {
							value: "t"
							loc: Object {
								filename: "esprima/invalid-syntax/migrated_0039/input.js"
								end: Object {
									column: 5
									line: 1
								}
								start: Object {
									column: 4
									line: 1
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0039/input.js:1:5 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unterminated regular expression

    /test
         ^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```