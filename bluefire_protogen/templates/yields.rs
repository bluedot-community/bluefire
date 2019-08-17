{% for yeeld in api.yields %}
    {% let yield_name = yeeld.name.camel_case() + "Yield" %}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct {{ yield_name }} {
        {% for arg in yeeld.args %}
            pub {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
        {% endfor %}
    }

    impl {{ yield_name }} {
        pub fn get_code(&self) -> http::StatusCode {
            {{ yeeld.code.rust_format() }}
        }
    }
{% endfor %}
