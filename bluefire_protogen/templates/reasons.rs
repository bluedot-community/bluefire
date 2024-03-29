{% for reason in api.reasons %}
    {% let name = reason.name.camel_case() + "Reason" %}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(tag = "reason")]
    pub enum {{ name }} {
        {% for case in reason.cases %}
            #[serde(rename = "{{ case.name.snake_case() }}")]
            {% if case.args.len() > 0 %}
                {{ case.name.camel_case() }} {
                    {% for arg in case.args %}
                        {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
                    {% endfor %}
                },
            {% else %}
                {{ case.name.camel_case() }},
            {% endif %}
        {% endfor %}
    }

    impl {{ name }} {
        {% for case in reason.cases %}
            pub fn new_{{ case.name.snake_case() }}(
            {% if case.args.len() > 0 %}
                {% for arg in case.args %}
                    {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
                {% endfor %}
            {% endif %}
            ) -> Self {
                {{ name }}::{{ case.name.camel_case() }}
                {% if case.args.len() > 0 %}
                    {
                    {% for arg in case.args %}
                        {{ arg.name().snake_case() }},
                    {% endfor %}
                    }
                {% endif %}
            }
        {% endfor %}

        pub fn get_code(&self) -> http::StatusCode {
            match self {
                {% for case in reason.cases %}
                    {{ name }}::{{ case.name.camel_case() }} { .. } => {{ case.code.rust_format() }},
                {% endfor %}
            }
        }
    }

    {# TODO: Remove this `impl`. Responses should be constructed from `*Response`s only. #}
    impl From<{{ name }}> for bluefire_backend::Response {
        fn from(reason: {{ name }}) -> bluefire_backend::Response {
            let mut value = serde_json::Map::new();
            value.insert("result".to_string(), serde_json::Value::String("{{ reason.variant.as_str() }}".to_string()));
            value.insert("content".to_string(), serde_json::to_value(&reason).expect("Serialize response to JSON Value"));

            http::response::Builder::new()
                .status(reason.get_code())
                .body(serde_json::to_string(&value).expect("Serialize response to JSON").into_bytes())
                .expect("Build response")
        }
    }
{% endfor %}
