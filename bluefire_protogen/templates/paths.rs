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

        {# Constructor from map #}
        pub fn new_from_map(
            _map: &std::collections::HashMap<&'static str, String>,
        ) -> Result<Self, &'static str> {
            Ok(Self {
                {% for segment in path.segments %}
                    {% match segment %}
                        {% when spec::Segment::Exact with (_) %}
                            {# nothing to generate #}
                        {% when spec::Segment::Str with (name) %}
                            {{ name.snake_case() }}: {
                                if let Some(value) = _map.get("{{ name.snake_case() }}") {
                                    value.clone()
                                } else {
                                    return Err("{{ name.snake_case() }}");
                                }
                            },
                    {% endmatch %}
                {% endfor %}
            })
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

    impl std::convert::TryFrom<&std::collections::HashMap<&'static str, String>> for {{ name }} {
        type Error = &'static str;
        fn try_from(
            map: &std::collections::HashMap<&'static str, String>,
        ) -> Result<{{ name }}, Self::Error> {
            {{ name }}::new_from_map(map)
        }
    }
{% endfor %}
