# mongojson-converter-cli

**mongojson-converter** is a CLI utility written in Rust that converts MongoDB-exported JSON into clean, standard JSON. It transforms special MongoDB types like `NumberDouble`, `NumberInt`, `NumberLong`,`oid` and `date` to `ISODate` into their native JSON equivalents, making the data easier to work with in systems expecting valid JSON.

## ✨ Features

- Converts MongoDB-extended JSON (BSON-like) into standard JSON
- Fixes:
  - `ISODate(...)` → ISO 8601 date strings
  - `NumberInt(...)`, `NumberLong(...)` → JSON numbers
- Handles nested structures and arrays
- Fast and lightweight (compiled Rust binary)
- command - ./mongo-json-coverter-cli src=example/1.json o=example/output.json
