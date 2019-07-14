{# ROUTE #}

bluefire_backend::router::Route::
{%- match route.segment -%}
    {%- when spec::Segment::Exact with (name) -%}
        exact("{{ name.snake_case() }}")
    {%- when spec::Segment::Str with (name) -%}
        param("{{ name.snake_case() }}")
{%- endmatch -%}

{%- match route.name -%}
    {%- when Some with (name) -%}
        .with_view(Box::new({{ name.camel_case() }}View))
    {%- when None -%}
{%- endmatch -%}

{%- if route.routes.len() > 0 -%}
    .with_routes(vec![
        {%- for route in route.routes -%}
            {{ generator.route(route) }},
        {%- endfor -%}
    ])
{%- endif -%}
