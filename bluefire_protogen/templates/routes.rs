bluefire_backend::router::Route::index()

{%- match routes.name -%}
    {%- when Some with (name) -%}
        .with_view(Box::new({{ name.snake_case() }}))
        {%- match routes.label_prefix -%}
            {%- when Some with (label_prefix) -%}
                .with_label("{{ label_prefix }}{{ name.snake_case() }}")
            {%- when None -%}
        {%- endmatch -%}
    {%- when None -%}
{%- endmatch -%}

{%- if routes.routes.len() > 0 -%}
    .with_routes(vec![
        {%- for route in routes.routes -%}
            {{ generator.route(route, routes.label_prefix) }},
        {%- endfor -%}
    ])
{%- endif -%}
