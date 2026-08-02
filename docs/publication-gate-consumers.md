# Publication gate consumers

ClaimWright's `publication check` report is an offline evidence surface for
GroundRecall, CiteGeist, and accountable publication workflows. Consumers may
store and display findings, route unresolved work for human review, and retain
the report with the release record.

The `decision` is not a permission grant. In particular, consumers must never
convert `hard_gate` to `allow`, infer originality from a similarity score, or
treat a finding as a misconduct adjudication. A separate human publication
approval remains required after an integrity `pass`.

The versioned fixture at
`fixtures/groundrecall/publication_gate_mcp_responses.json` defines the planned
future `claimwright.publication_gate` MCP response shape. It is a fixture only;
no MCP server or network adapter is implemented by this roadmap.

CI should propagate ClaimWright exit status unchanged: 0 permits the next
review stage, while 1 (hard gate/deny), 2 (invalid input), and 3 (I/O failure)
stop the workflow.
