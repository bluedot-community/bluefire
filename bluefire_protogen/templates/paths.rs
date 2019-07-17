{% for path in paths %}
    {% let name = path.name.camel_case() + "PathParams" %}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct {{ name }} {
        {% for segment in path.segments %}
            {% match segment %}
                {% when spec::Segment::Exact with (_) %}
                    {# nothing to generate #}
                {% when spec::Segment::Str with (name) %}
                    pub {{ name.snake_case() }}: String,
            {% endmatch %}
        {% endfor %}
    }

    impl {{ name }} {
        {# Constructor #}
        pub fn new (
            {% for segment in path.segments %}
                {% match segment %}
                    {% when spec::Segment::Exact with (_) %}
                        {# nothing to generate #}
                    {% when spec::Segment::Str with (name) %}
                        {{ name.snake_case() }}: String,
                {% endmatch %}
            {% endfor %}
        ) -> Self {
            Self {
                {% for segment in path.segments %}
                    {% match segment %}
                        {% when spec::Segment::Exact with (_) %}
                            {# nothing to generate #}
                        {% when spec::Segment::Str with (name) %}
                            {{ name.snake_case() }},
                    {% endmatch %}
                {% endfor %}
            }
        }

        {# Path construction #}
        pub fn to_path(&self) -> String {
            String::new()
            {% for segment in path.segments %}
                {% match segment %}
                    {% when spec::Segment::Exact with (name) %}
                        + "/{{ name.snake_case() }}"
                    {% when spec::Segment::Str with (name) %}
                        + "/" + &self.{{ name.snake_case() }}
                {% endmatch %}
            {% endfor %}
        }

        {# Associated path construction #}
        pub fn get(
            {% for segment in path.segments %}
                {% match segment %}
                    {% when spec::Segment::Exact with (_) %}
                        {# nothing to generate #}
                    {% when spec::Segment::Str with (name) %}
                        {{ name.snake_case() }}: &str,
                {% endmatch %}
            {% endfor %}
        ) -> String {
            String::new()
            {% for segment in path.segments %}
                {% match segment %}
                    {% when spec::Segment::Exact with (name) %}
                        + "/{{ name.snake_case() }}"
                    {% when spec::Segment::Str with (name) %}
                        + "/" + {{ name.snake_case() }}
                {% endmatch %}
            {% endfor %}
        }
    }
{% endfor %}
