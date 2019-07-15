use serde_derive::{Serialize, Deserialize};

{# TYPES #}

{% for tipe in api.types %}
    {% match tipe.container %}
        {% when spec::TypeRepr::Simple with {simple_type, validation} %}
            pub type {{ tipe.name.camel_case() }} = {{ simple_type.rust_format() }};

            {% match validation %}
                {% when Some with (validation) %}
                    pub enum {{ tipe.name.camel_case() }}ValidationResult {
                        Ok,
                        {% for check in validation.checks %}
                            {{ check.get_error_name().camel_case() }},
                        {% endfor %}
                        {% for condition in validation.conditions %}
                            {{ condition.get_error_name().camel_case() }},
                        {% endfor %}
                    }

                    pub fn __validate_{{ tipe.name.snake_case() }}(item: &{{ tipe.name.camel_case() }})
                    -> bluefire_twine::ValidationResult<{{ tipe.name.camel_case() }}ValidationResult> {
                        let mut validation_result = bluefire_twine::ValidationResult::new();

                        {% for condition in validation.conditions %}
                            {% match condition %}
                                {% when spec::Condition::Le with (value) %}
                                    {% match simple_type %}
                                        {% when spec::SimpleType::U8 %}
                                            if *item > {{ value }} {
                                        {% when spec::SimpleType::U32 %}
                                            if *item > {{ value }} {
                                        {% when spec::SimpleType::I32 %}
                                            if *item > {{ value }} {
                                        {% when spec::SimpleType::F32 %}
                                            if *item > {{ "{:.4}"|format(value) }} {
                                        {% when spec::SimpleType::F64 %}
                                            if *item > {{ "{:.4}"|format(value) }} {
                                        {% when spec::SimpleType::Str %}
                                            {# nothing to generate - this type cannot be compared #}
                                        {% when spec::SimpleType::Id %}
                                            {# nothing to generate - this type cannot be compared #}
                                    {% endmatch %}
                                {% when spec::Condition::Ge with (value) %}
                                    {% match simple_type %}
                                        {% when spec::SimpleType::U8 %}
                                            if *item < {{ value }} {
                                        {% when spec::SimpleType::U32 %}
                                            if *item < {{ value }} {
                                        {% when spec::SimpleType::I32 %}
                                            if *item < {{ value }} {
                                        {% when spec::SimpleType::F32 %}
                                            if *item < {{ "{:.4}"|format(value) }} {
                                        {% when spec::SimpleType::F64 %}
                                            if *item < {{ "{:.4}"|format(value) }} {
                                        {% when spec::SimpleType::Str %}
                                            {# nothing to generate - this type cannot be compared #}
                                        {% when spec::SimpleType::Id %}
                                            {# nothing to generate - this type cannot be compared #}
                                    {% endmatch %}
                                {% when spec::Condition::LenEq with (len) %}
                                    {% match simple_type %}
                                        {% when spec::SimpleType::Str %}
                                            if item.len() != {{ len }} {
                                        {% when _ %}
                                            {# nothing to generate - this type does not have length #}
                                    {% endmatch %}
                                {% when spec::Condition::LenLe with (len) %}
                                    {% match simple_type %}
                                        {% when spec::SimpleType::Str %}
                                            if item.len() > {{ len }} {
                                        {% when _ %}
                                            {# nothing to generate - this type does not have length #}
                                    {% endmatch %}
                                {% when spec::Condition::LenGe with (len) %}
                                    {% match simple_type %}
                                        {% when spec::SimpleType::Str %}
                                            if item.len() < {{ len }} {
                                        {% when _ %}
                                            {# nothing to generate - this type does not have length #}
                                    {% endmatch %}
                            {% endmatch %}
                                validation_result.add(
                                    {{ tipe.name.camel_case() }}ValidationResult::{{ condition.get_error_name().camel_case() }}
                                );
                            }
                        {% endfor %}

                        {% for check in validation.checks %}
                            {% match check %}
                                {% when spec::Check::Email %}
                                    if !bluefire_twine::validation::validate_email(item) {
                                        validation_result.add(
                                            {{ tipe.name.camel_case() }}ValidationResult::{{ check.get_error_name().camel_case() }}
                                        );
                                    }
                            {% endmatch %}
                        {% endfor %}

                        validation_result
                    }
                {% when None %}
            {% endmatch %}
        {% when spec::TypeRepr::External %}
            {# nothing to generate #}
        {% when spec::TypeRepr::Struct with {members} %}
            #[derive(Clone, Debug, Serialize, Deserialize)]
            pub struct {{ tipe.name.camel_case() }} {
                {% for member in members %}
                    pub {{ member.name().snake_case() }}: {{ member.rust_type() }},
                {% endfor %}
            }

            impl {{ tipe.name.camel_case() }} {
                pub fn new(
                    {% for member in members %}
                        {{ member.name().snake_case() }}: {{ member.rust_type() }},
                    {% endfor %}
                ) -> Self {
                    Self {
                        {% for member in members %}
                            {{ member.name().snake_case() }},
                        {% endfor %}
                    }
                }
            }
        {% when spec::TypeRepr::Union with {members} %}
            #[derive(Clone, Debug, Serialize, Deserialize)]
            #[serde(tag = "variant", content = "content")]
            pub enum {{ tipe.name.camel_case() }} {
                {% for member in members %}
                    #[serde(rename = "{{ member.name().snake_case() }}")]
                    {{ member.name().camel_case() }}({{ member.rust_type() }}),
                {% endfor %}
            }
        {% when spec::TypeRepr::Enum with {values} %}
            #[derive(Clone, Debug, Serialize, Deserialize)]
            pub enum {{ tipe.name.camel_case() }} {
                {% for value in values %}
                    #[serde(rename = "{{ value.snake_case() }}")]
                    {{ value.camel_case() }},
                {% endfor %}
            }

            impl {{ tipe.name.camel_case() }} {
                pub fn to_str(&self) -> &'static str {
                    match &self {
                        {% for value in values %}
                            {{ tipe.name.camel_case() }}::{{ value.camel_case() }} => "{{ value.snake_case() }}",
                        {% endfor %}
                    }
                }

                pub fn from_str(text: &str) -> Option<Self> {
                    match text {
                        {% for value in values %}
                            "{{ value.snake_case() }}" => Some({{ tipe.name.camel_case() }}::{{ value.camel_case() }}),
                        {% endfor %}
                        _ => None
                    }
                }
            }
    {% endmatch %}
{% endfor %}
