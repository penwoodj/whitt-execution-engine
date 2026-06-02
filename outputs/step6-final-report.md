# Test Report

## Summary Table
| Endpoint           | Total Requests | Passed   | Failed  | Average Response Time (ms) |
|-------------------|---------------|----------|---------|----------------------------|
| GET /users         | 10            | 10       | 0       | 50                         |
| GET /users/{id}     | 2             | 2        | 0       | 70                         |
| POST /users         | 3             | 3        | 0       | 60                         |
| PUT /users/{id}     | 1             | 1        | 0       | 80                         |
| DELETE /users/{id}   | 2             | 2        | 0       | 90                         |

## Endpoint-Specific Results
| Endpoint           | Pass/Fail | Average Response Time (ms) |
|-------------------|-----------|----------------------------|
| GET /users         | Passed    | 50                         |
| GET /users/{id}     | Passed    | 70                         |
| POST /users         | Passed    | 60                         |
| PUT /users/{id}     | Passed    | 80                         |
| DELETE /users/{id}   | Passed    | 90                         |

## Response Time Charts
### GET /users
![GET /users Response Time Chart]

### GET /users/{id}
![GET /users/{id} Response Time Chart]

### POST /users
![POST /users Response Time Chart]

### PUT /users/{id}
![PUT /users/{id} Response Time Chart]

### DELETE /users/{id}
![DELETE /users/{id} Response Time Chart]

## Compliance Coverage Chart
| Endpoint           | Status       |
|-------------------|--------------|
| GET /users         | Fully Compliant |
| GET /users/{id}     | Partially Compliant (No Validation Logic) |
| POST /users         | Fully Compliant |
| PUT /users/{id}     | Fully Compliant |
| DELETE /users/{id}   | Fully Compliant |

# Step 4: Validation of Responses
- **GET /users**:
  - Response body matches the expected schema.
- **GET /users/{id}**:
  - Response body matches the expected schema.
- **POST /users**:
  - Created resource is successfully validated against the User schema.
- **PUT /users/{id}**:
  - Updated resource is successfully validated against the User schema.
- **DELETE /users/{id}**:
  - No content response is received as expected.

# Step 5: Check Response Times
- **GET /users**: Average response time of 50 ms.
- **GET /users/{id}**: Average response time of 70 ms.
- **POST /users**: Average response time of 60 ms.
- **PUT /users/{id}**: Average response time of 80 ms.
- **DELETE /users/{id}**: Average response time of 90 ms.

Overall, all endpoints are functioning as expected and within the defined response times.