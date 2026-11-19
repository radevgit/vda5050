# vda5050

Rust types for the messages of [VDA 5050](https://github.com/VDA5050/VDA5050) **version 3.0.1**, serialisable to and from the JSON the standard defines.

- All 8 message types: `connection`, `factsheet`, `instantActions`, `order`, `responses`, `state`, `visualization`, `zoneSet`.
- Field names and enum strings match the official JSON schemas exactly.
- Optional fields are `Option` and are left out of the JSON when `None`.
- Only `serde` and `serde_json` as dependencies.
- Tests check that example messages, and what this crate serialises from them, validate against the official schemas (vendored in `spec/`, see `spec/SOURCE.md`).

Not included: MQTT transport, topic handling, simulation or control logic.

## Example

```rust
use vda5050::order::Order;

fn main() -> Result<(), serde_json::Error> {
    // An order as received from fleet control.
    let json = r#"{
        "headerId": 1, "timestamp": "2026-11-19T10:00:00.000Z", "version": "3.0.1",
        "manufacturer": "ACME", "serialNumber": "R-001",
        "orderId": "o-1", "orderUpdateId": 0,
        "nodes": [
            {"nodeId": "n1", "sequenceId": 0, "released": true, "actions": []},
            {"nodeId": "n2", "sequenceId": 2, "released": true, "actions": []}
        ],
        "edges": [
            {"edgeId": "e1", "sequenceId": 1, "released": true, "maximumSpeed": 1.5, "actions": []}
        ]
    }"#;

    let order: Order = serde_json::from_str(json)?;
    println!("order {} has {} nodes", order.order_id, order.nodes.len());
    assert_eq!(order.edges[0].maximum_speed, Some(1.5));

    // Types serialise back to the exact JSON field names.
    let back = serde_json::to_string(&order)?;
    assert!(back.contains("\"orderUpdateId\":0"));
    Ok(())
}
```

## Version

Follows VDA 5050 **3.0.1** (branch `dev/3.0.1` of the official repository, commit `9176e22`, 2026-11-11). If the document and the schemas differ, the document is leading; this crate follows the schemas and fixes differences as they are found.

## Licence

MIT. The vendored specification in `spec/` is MIT-licensed by the Verband der Automobilindustrie (`spec/LICENSE-VDA5050`).
