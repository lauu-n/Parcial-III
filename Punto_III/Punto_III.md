# Comparación desempeño entre Python y RUST

## Python

Se implemetó en el código:

Además de funciones para graficar y mejorar el entendimiento visual como:

Resultados:
![alt text](python_metrics-1.png)

## RUST

Se implemetó en el código:

Además de funciones para graficar y mejorar el entendimiento visual como:

Para ejecutar:
    1. Crear la estructura del proyecto Cargo:
        En tu terminal, ejecuta:
        ```
        cargo new linear_regression_rust
        cd linear_regression_rust
        ```
    2. Configurar Cargo.toml
        Edita el archivo Cargo.toml que se creó automáticamente y reemplaza su contenido con:
        ```
        [package]
        name = "linear_regression_rust"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        plotters = "0.3.5"
        sysinfo = "0.29.0"
        ```
    3. Compilar y ejecutar
        ```
        cargo run --release
        ```

Resultados:


## Conclusiones