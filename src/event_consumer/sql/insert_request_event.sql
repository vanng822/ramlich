INSERT INTO request_event(id, url, requested_at, response_time, status_code)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields;