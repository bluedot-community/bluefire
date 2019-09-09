bluefire_backend::router::Route::
{%- match route.segment -%}
    {%- when spec::Segment::Exact with (name) -%}
        exact("{{ name.snake_case() }}")
    {%- when spec::Segment::Str with (name) -%}
        param("{{ name.snake_case() }}")
{%- endmatch -%}

{%- match route.name -%}
    {%- when Some with (name) -%}
        .with_view(Box::new({{ name.snake_case() }}))
        {%- match label_prefix -%}
            {%- when Some with (label_prefix) -%}
                .with_label("{{ label_prefix }}{{ name.snake_case() }}")
            {%- when None -%}
        {%- endmatch -%}
    {%- when None -%}
{%- endmatch -%}

{%- if route.routes.len() > 0 -%}
    .with_routes(vec![
        {%- for route in route.routes -%}
            {{ generator.route(route, label_prefix) }},
        {%- endfor -%}
    ])
{%- endif -%}
