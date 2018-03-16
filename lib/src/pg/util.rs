use postgres::types::ToSql;

fn get_from_table_name(root_table_name: &str, table_number: usize) -> String {
    match table_number {
        0 => root_table_name.to_string(),
        _ => format!("pipe_{}", table_number),
    }
}

fn format_query(template: &str, from_table_name: &str, cur_params_length: usize, param_number: usize) -> String {
    // Turn the query template into an actual query.
    // TODO: This could be made much more efficient.
    let mut query = template.replace("%t", from_table_name);

    for i in 1..cur_params_length + 1 {
        query = query.replacen("%p", &format!("${}", param_number + i)[..], 1);
    }

    query
}

pub struct CTEQueryBuilder {
    queries: Vec<String>,
    params: Vec<Box<ToSql>>,
}

impl CTEQueryBuilder {
    pub fn new() -> CTEQueryBuilder {
        CTEQueryBuilder {
            queries: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn push(&mut self, query_template: &str, root_table_name: &str, params: Vec<Box<ToSql>>) {
        // TODO: because we don't support query parameter numbers, there are a
        // couple of times where we have to pass the same parameter multiple
        // times. Fix this.

        let from_table_name = get_from_table_name(root_table_name, self.queries.len());
        let query = format_query(
            query_template,
            &from_table_name[..],
            params.len(),
            self.params.len(),
        );
        self.queries.push(query);
        self.params.extend(params);
    }

    pub fn into_query_payload(self, query_template: &str, params: Vec<Box<ToSql>>) -> (String, Vec<Box<ToSql>>) {
        if self.queries.is_empty() {
            panic!("No queries");
        }

        let from_table_name = get_from_table_name("", self.queries.len());
        let query = format_query(
            query_template,
            &from_table_name[..],
            params.len(),
            self.params.len(),
        );

        let mut full_params = self.params;
        full_params.extend(params);

        let mut buffer: Vec<String> = Vec::new();
        buffer.push("WITH ".to_string());

        for (i, query) in self.queries.into_iter().enumerate() {
            if i > 0 {
                buffer.push(", ".to_string());
            }

            buffer.push(format!("pipe_{} AS (", i + 1));
            buffer.push(query);
            buffer.push(")".to_string());
        }

        buffer.push(" ".to_string());

        buffer.push(query);
        let full_query = buffer.join("");
        (full_query, full_params)
    }
}
