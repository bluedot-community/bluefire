/// Provides info about static files.
pub struct {{ info.struct_name }} {
    {% for source in spec.sources %}
        {% match source.variant %}
            {% when Type::Js with { field_name } %}
                /// `/{}/{{ config.namespace }}/{{ source.output_base_name }}.js`
                pub {{ field_name }}: String,
            {% when Type::Scss with { field_name } %}
                /// `/{}/{{ config.namespace }}/{{ source.output_base_name }}.css`
                pub {{ field_name }}: String,
            {% when Type::Wasm with { field_name_wasm, field_name_js, target_path } %}
                /// `/{}/{{ config.namespace }}/{{ source.output_base_name }}.wasm`
                pub {{ field_name_wasm }}: String,
                /// `/{}/{{ config.namespace }}/{{ source.output_base_name }}.js`
                pub {{ field_name_js }}: String,
        {% endmatch %}
    {% endfor %}
}

impl {{ info.struct_name }} {
    /// Constructs a new `{{ info.struct_name }}`.
    pub fn new(static_root: &str) -> Self {
        Self {
            {% for source in spec.sources %}
                {% match source.variant %}
                    {% when Type::Js with { field_name } %}
                        {{ field_name }}: format!("/{}/{{ config.namespace }}/{{ source.output_base_name }}.js", static_root),
                    {% when Type::Scss with { field_name } %}
                        {{ field_name }}: format!("/{}/{{ config.namespace }}/{{ source.output_base_name }}.css", static_root),
                    {% when Type::Wasm with { field_name_wasm, field_name_js, target_path } %}
                        {{ field_name_wasm }}: format!("/{}/{{ config.namespace }}/{{ source.output_base_name }}.wasm", static_root),
                        {{ field_name_js }}: format!("/{}/{{ config.namespace }}/{{ source.output_base_name }}.js", static_root),
                {% endmatch %}
            {% endfor %}
        }
    }

    /// Builds a route for the static files.
    pub fn make_route() -> bluefire_backend::router::Route {
        use bluefire_backend::{router::Route, static_files::StaticHandler};
        let mut route = Route::exact("{{ config.namespace }}");

        {% for source in spec.sources %}
            {% for (input_path, output_name, content_type) in generator.make_paths(source) %}
                let data = include_bytes!("{{ input_path }}").to_vec();
                let handler = Box::new(StaticHandler::new(data, "{{ content_type }}".to_string()));
                route.add_route(Route::exact("{{ output_name }}").with_view(handler));
            {% endfor %}
        {% endfor %}

        route
    }
}
