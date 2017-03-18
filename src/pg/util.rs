use postgres::error as pg_error;
use postgres::types::ToSql;

pub fn pg_error_to_description(err: pg_error::Error) -> String {
    match err {
        pg_error::Error::Db(err) => {
            match err.detail {
                Some(ref detail) => format!("[{}] {}: {}", err.code.code(), err.message, detail),
                None => format!("[{}] {}", err.code.code(), err.message),
            }
        }
        pg_error::Error::Io(_) => "Could not communicate with the database instance".to_string(),
        pg_error::Error::Conversion(err) => panic!(err),
    }
}

pub struct CTEQueryBuilder {
    table_number: usize,
    param_number: usize,
    queries: Vec<String>,
    params: Vec<Box<ToSql>>
}

impl CTEQueryBuilder {
    pub fn new() -> CTEQueryBuilder {
        return CTEQueryBuilder{
            table_number: 0,
            param_number: 0,
            queries: Vec::new(),
            params: Vec::new()
        }
    }

    pub fn push(&mut self, query_template: &str, root_table_name: &str, params: Vec<Box<ToSql>>) {
        let from_table_name = match self.table_number {
            0 => root_table_name.to_string(),
            _ => format!("pipe_{}", self.table_number)
        };

        self.table_number += 1;

        // Turn the query template into an actual query.
        // TODO: This could be made much more efficient.
        let mut query = query_template.replacen("%t", &from_table_name[..], 1);

        for _ in 0..params.len() {
            query = query.replacen("%p", &format!("${}", self.param_number + 1)[..], 1);
            self.param_number += 1;
        }

        self.queries.push(query);
        self.params.extend(params);
    }

    pub fn to_query_payload(self) -> (String, Vec<Box<ToSql>>) {
        match self.queries.len() {
            0 => panic!("No queries"),
            1 => (self.queries[0].to_string(), self.params),
            _ => {
                let mut buffer: Vec<String> = Vec::new();
                let mut i = 0;
                buffer.push("WITH ".to_string());

                for query in self.queries.into_iter() {
                    if i > 0 {
                        buffer.push(", ".to_string());
                    }

                    i += 1;
                    buffer.push(format!("pipe_{} AS (", i));
                    buffer.push(query);
                    buffer.push(")".to_string());
                }

                buffer.push(format!(" SELECT * FROM pipe_{}", i));
                let full_query = buffer.join("");
                (full_query, self.params)
            }
        }
    }
}
