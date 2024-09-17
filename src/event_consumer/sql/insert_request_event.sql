INSERT INTO request_event(id, url, requested_at, response_time)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;