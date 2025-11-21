# 1. Diseño para realizar regresión lineal con Paradigma de Concurrencia y Cálculo PI

## Arquitectura:
  - Coordinador principal: Controla iteraciones y sincroniza.
  - Workers: Para trabajar en concurrente, calculan partes de los gradientes.
  - Shared Buffer: Almacena parámetros y resultados durante el proceso.

## Componentes:
  1.	Main controller:
    -	Inicia parámetros (w, b).
    -	Divide el dataset en chunks (segmentos).
    -	Coordina epochs.
    -	Sincroniza resultados.
  2.	Gradient workers: 
    -	Cada worker calcula gradientes para subconjuntos de datos.
    -	Calcula cada dw_partial y db_partial para su chunk.
    -	Enviar resultados para coordinar.
  3.	Server:
    -	Almacena w y b actuales.
    -	Agrega gradientes de todos los workers.
    -	Actualiza gradientes.

## Flujo:
```
Para cada epoch:
  1. Main Controller distribuye (w, b) actuales a todos los workers
  2. Cada worker procesa su chunk de datos:
     - Calcula y_pred = w * X_chunk + b
     - Calcula error = y_pred - y_chunk
     - Calcula dw_partial = (2/m_chunk) * sum(error * X_chunk)
     - Calcula db_partial = (2/m_chunk) * sum(error)
  3. Workers envían gradientes parciales al Parameter Server
  4. Parameter Server agrega gradientes:
     dw_total = promedio(dw_partial_1, dw_partial_2, ..., dw_partial_N)
     db_total = promedio(db_partial_1, db_partial_2, ..., db_partial_N)
  5. Actualiza parámetros:
     w = w - learning_rate * dw_total
     b = b - learning_rate * db_total
  6. Repite hasta convergencia o epochs completados
```
