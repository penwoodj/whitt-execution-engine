| ID | Hypothesis | Evidence for | Evidence against | Test to write | Verdict |
|----|-----------|--------------|------------------|---------------|---------|
| H1 | External app consumed VRAM/RAM mid-inference |  |  | watchdog red-state test | |
| H2 | ctx/KV budget exceeded VRAM (config drift) |  |  | admit_load rejection test | |
| H3 | Concurrency >1 on 8GB card |  |  | detect_max_concurrent test | |
| H4 | Server slot error as HTTP 500, engine continued |  |  | step-fails propagation test | |
| H5 | Sensor unit bug (bytes vs KB) |  |  | sensor parser test | |
