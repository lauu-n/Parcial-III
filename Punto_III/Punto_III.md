# Comparación desempeño entre Python y RUST

## Metodología
Para realizar una comparación justa y significativa, se implementó el algoritmo de regresión lineal en ambos lenguajes bajo las mismas condiciones:
- Mismo dataset: X = [1, 2, 3, 4, 5], y = [2, 4, 6, 8, 10].
- Mismos hiperparámetros: learning_rate = 0.01, epochs = 1000.
- Mismas métricas: MSE, tiempo de ejecución, uso de memoria, evolución de parámetros.
- Mismo hardware: Todas las pruebas ejecutadas en la misma máquina.

## Python
Se implemetó en el código:
- NumPy para operaciones vectorizadas eficientes.
- Matplotlib para visualización de resultados.
- Tracemalloc para medición precisa de memoria.
- Time para medición de tiempos de ejecución.

Además de funciones para graficar y mejorar el entendimiento visual como:
- Evolución del Error (MSE): Seguimiento de la convergencia del algoritmo.
- Evolución de Parámetros: Comportamiento de w (pendiente) y b (intercepto).
- Uso de Memoria: Consumo de RAM durante el entrenamiento.
- Gráfica de Convergencia: Relación entre el valor de w y el error.

Resultados:
<img width="3570" height="2966" alt="python_metrics" src="https://github.com/user-attachments/assets/af80e290-aa3e-464a-af0d-b646f772cde6" />

✓ Ventajas:
- Desarrollo más rápido y código más conciso.
- Gran cantidad de librerías.
- Fácil debugging.
- Menor consumo de memoria en este caso.

✗ Desventajas:
- Rendimiento significativamente menor.
- Overhead del interprete y garbage collector.
- Dependencia de librerías externas para performance.

## RUST
Se implemetó en el código:
- Cargo como gestor de paquetes y build system.
- Plotters para generación de gráficas nativas.
- Sysinfo para monitoreo del sistema.
- Std::time para medición de performance

Además de funciones para graficar y mejorar el entendimiento visual como:
- Evolución del Error: MSE vs épocas de entrenamiento.
- Parámetros del Modelo: Trayectoria de w y b durante el entrenamiento.
- Consumo de Memoria: Uso de RAM en tiempo real.
- Análisis de Convergencia: Relación parámetro-error.

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
3. Crear el código Rust
Cambiar el contenido de src/main.rs por el de regresion_lineal.rs en el repositorio
4. Compilar y ejecutar
```
cargo run --release
```

✓ Ventajas:
- Rendimiento mucho más alto.
- Seguridad de memoria en tiempo de compilación.
- Sin dependencias externas.

✗ Desventajas:
- Curva de aprendizaje más pronunciada.
- Código más difícil de manejar.
- Mayor tiempo de desarrollo.
        
## Conclusiones
- Rendimiento: RUST es significativamente más rápido para cómputo, haciendo la diferencia en aplicaciones de producción a escala.
- Productividad vs Performance: Python ofrece mejor productividad para desarrollo, mientras RUST ofrece mejor performance en ejecución.
- Precisión Equivalente: Ambos lenguajes proveen precisión numérica equivalente para aplicaciones de machine learning.
- Overhead de Memoria: La implementación RUST consumió más memoria, pero este overhead puede justificarse por la ganancia en velocidad.
- Selección Contextual: La elección entre Python y RUST debe basarse en los requisitos específicos del proyecto: velocidad de desarrollo vs performance de ejecución.

Para regresión lineal y algoritmos similares de machine learning, RUST ofrece ventajas de performance sustanciales, mientras Python mantiene ventajas en velocidad de desarrollo y ecosistema. La decisión óptima depende del contexto específico de aplicación y los trade-offs aceptables entre desarrollo time y runtime performance.

