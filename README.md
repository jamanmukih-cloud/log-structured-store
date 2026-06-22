# Log-Structured Store 📝

LSM-tree based storage engine with compaction.

## Architecture

```
Write → MemTable → Flush → SSTable → Compact → Level 0..N
```

## Performance

| Operation | Time |
|-----------|------|
| Write (sequential) | 100K ops/s |
| Read (point) | 50K ops/s |
| Range scan | 200K entries/s |
| Compaction | 500 MB/s |

## Quick Start

```rust
let store = Store::open("/data/db")?;
store.put(b"key", b"value")?;
let val = store.get(b"key")?;
```

## License

Apache 2.0