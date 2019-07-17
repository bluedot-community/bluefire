{% for yeeld in api.yields %}
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct {{ yeeld.name.camel_case() }}Yield {
        {% for arg in yeeld.args %}
            pub {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
        {% endfor %}
    }
{% endfor %}
