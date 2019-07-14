{# ROUTES #}

bluefire_backend::router::Route::index()

{%- match routes.name -%}
    {%- when Some with (name) -%}
        .with_view(Box::new({{ name.camel_case() }}View))
    {%- when None -%}
{%- endmatch -%}

{%- if routes.routes.len() > 0 -%}
    .with_routes(vec![
        {%- for route in routes.routes -%}
            {{ generator.route(route) }},
        {%- endfor -%}
    ])
{%- endif -%}
