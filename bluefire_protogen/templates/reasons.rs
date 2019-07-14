{# REASONS #}

{% for reason in api.reasons %}
    {% let name = reason.name.camel_case() + "Reason" %}

    #[derive(Clone, Debug, Serialize, Deserialize)]
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
        pub fn get_code(&self) -> http::StatusCode {
            match self {
                {% for case in reason.cases %}
                    {{ name }}::{{ case.name.camel_case() }} { .. } => {{ case.code.rust_format() }},
                {% endfor %}
            }
        }
    }

    impl From<{{ name }}> for http::Response<String> {
        fn from(reason: {{ name }}) -> http::Response<String> {
            let mut value = serde_json::to_value(&reason).expect("Serialize response to JSON Value");
            let object = value.as_object_mut().expect("As JSON object");
            object.insert("_variant".to_string(), serde_json::Value::String("failure".to_string()));

            http::response::Builder::new()
                .status(reason.get_code())
                .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(serde_json::to_string(&value).expect("Serialize response to JSON"))
                .expect("Build response")
        }
    }
{% endfor %}
