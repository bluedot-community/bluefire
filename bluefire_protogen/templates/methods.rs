{% for method in api.methods %}
    {% let method_name = method.name.camel_case() + "Method" %}
    {% let request_name = method.name.camel_case() + "Request" %}
    {% let response_name = method.name.camel_case() + "Response" %}
    {% let path_name = method.request.path.camel_case() + "PathParams" %}

    {# REQUEST #}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct {{ request_name }} {
        {% for arg in method.request.args %}
            pub {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
        {% endfor %}
    }

    impl {{ request_name }} {
        pub fn new (
            {% for arg in method.request.args %}
                {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
            {% endfor %}
        ) -> Self {
            Self {
                {% for arg in method.request.args %}
                    {{ arg.name().snake_case() }},
                {% endfor %}
            }
        }

        {% if method.request.method == spec::HttpMethod::Get %}
            pub fn from_query_string(query_str: &str) -> Result<Self, serde_urlencoded::de::Error> {
                serde_urlencoded::from_str(query_str)
            }

            pub fn to_query_string(&self) -> Result<String, serde_urlencoded::ser::Error> {
                serde_urlencoded::to_string(self)
            }
        {% else %}
            pub fn from_json_string(json_str: &str) -> Result<Self, serde_json::Error> {
                serde_json::from_str(json_str)
            }

            pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(self)
            }
        {% endif %}

        pub fn get_method(&self) -> http::method::Method {
            {{ method.request.method.rust_format() }}
        }

        pub fn get_method_name(&self) -> &'static str {
            "{{ method.request.method.to_str() }}"
        }

        pub fn to_message(&self, params: &{{ path_name }}) -> bluefire_twine::Message {
            bluefire_twine::Message::new(
                self.get_method_name(),
                params.to_path(),
                {% if method.request.method == spec::HttpMethod::Get %}
                    self.to_query_string().expect("Cast to query string"),
                    String::new(),
                {% else %}
                    String::new(),
                    self.to_json_string().expect("Cast to json string"),
                {% endif %}
            )
        }

        {% for member in method.request.args %}
            {% match member %}
                {% when spec::Member::Simple with {name, tipe} %}
                    {# nothing to generate #}
                {% when spec::Member::Contained with {name, tipe, container} %}
                    {# nothing to generate #}
                {% when spec::Member::Defined with {name, tipe} %}
                    {% let found = generator.find_type(tipe, api.types) %}
                    {% match found.container %}
                        {% when spec::TypeRepr::Simple with {simple_type, validation} %}
                            {% if validation.is_some() %}
                                pub fn validate_{{ member.name().snake_case() }}(&self)
                                -> bluefire_twine::ValidationResult<{{ found.name.camel_case() }}ValidationResult> {
                                    __validate_{{ found.name.snake_case() }}(&self.{{ member.name().snake_case() }})
                                }
                            {% endif %}
                        {% when spec::TypeRepr::External %}
                            {# nothing to generate #}
                        {% when spec::TypeRepr::Struct with {members} %}
                            {# nothing to generate #}
                        {% when spec::TypeRepr::Union with {members} %}
                            {# nothing to generate #}
                        {% when spec::TypeRepr::Enum with {values} %}
                            {# nothing to generate #}
                    {% endmatch %}
            {% endmatch %}
        {% endfor %}
    }

    {# RESPONSE #}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(tag = "result", content = "content")]
    pub enum {{ response_name }} {
        #[serde(rename = "success")]
        Success({{ method.response.success.camel_case() }}Yield),
        {% match method.response.failure %}
            {% when Some with (failure) %}
                #[serde(rename = "failure")]
                Failure({{ failure.camel_case() }}Reason),
            {% when None %}
        {% endmatch %}
        #[serde(rename = "error")]
        Error({{ method.response.error.camel_case() }}Reason),
    }

    impl {{ response_name }} {
        {% let yeeld = generator.find_yield(method.response.success.clone(), api.yields) %}
        pub fn success(
            {% for arg in yeeld.args %}
                {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
            {% endfor %}
        ) -> (http::StatusCode, {{ response_name }}) {(
            {{ yeeld.code.rust_format() }},
            {{ response_name }}::Success({{ yeeld.name.camel_case() }}Yield {
                {% for arg in yeeld.args %}
                    {{ arg.name().snake_case() }},
                {% endfor %}
            })
        )}

        {% match method.response.failure %}
            {% when Some with (failure) %}
                {% let failure_reason = generator.find_reason(failure.clone(), api.reasons) %}
                {% for case in failure_reason.cases %}
                    pub fn failure_{{ case.name.snake_case() }}(
                        {% for arg in case.args %}
                            {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
                        {% endfor %}
                    ) -> (http::StatusCode, {{ response_name }}) {(
                        {{ case.code.rust_format() }},
                        {{ response_name }}::Failure({{ failure_reason.name.camel_case() }}Reason::{{ case.name.camel_case() }} {
                            {% for arg in case.args %}
                                {{ arg.name().snake_case() }},
                            {% endfor %}
                        })
                    )}
                {% endfor %}
            {% when None %}
        {% endmatch %}

        {% let error_reason = generator.find_reason(method.response.error.clone(), api.reasons) %}
        {% for case in error_reason.cases %}
            pub fn error_{{ case.name.snake_case() }}(
                {% for arg in case.args %}
                    {{ arg.name().snake_case() }}: {{ arg.rust_type() }},
                {% endfor %}
            ) -> (http::StatusCode, {{ response_name }}) {(
                {{ case.code.rust_format() }},
                {{ response_name }}::Error({{ error_reason.name.camel_case() }}Reason::{{ case.name.camel_case() }} {
                    {% for arg in case.args %}
                        {{ arg.name().snake_case() }},
                    {% endfor %}
                })
            )}
        {% endfor %}
    }

    {# METHODS #}

    pub struct {{ method_name }};

    impl bluefire_twine::Method for {{ method_name }} {
        type PathParams = {{ path_name }};
        type Request = {{ request_name }};
        type Response = {{ response_name }};
    }
{% endfor %}
