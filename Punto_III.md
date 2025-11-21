# Diseño Detallado de Aspectos

## Aspecto 1: LoggingAspect
Responsabilidad: Registrar el progreso del entrenamiento y métricas importantes

### Pointcuts:
- trainingStart(): Inicio del entrenamiento
- epochCompletion(epoch, mse, w, b): Finalización de cada época
- trainingEnd(w, b, totalTime): Finalización del entrenamiento

### Advices:
- before trainingStart(): Log inicial con hiperparámetros
- after epochCompletion(): Log cada N épocas con métricas
- after trainingEnd(): Log final con resultados y tiempo

### Interfaces:
```java
interface LoggingAspect {
    void logTrainingStart(hyperparams);
    void logEpochProgress(epoch, metrics);
    void logTrainingCompletion(results);
}
```

## Aspecto 2: ValidationAspect
Responsabilidad: Validar integridad y calidad de los datos

### Pointcuts:
- dataInput(X, y): Entrada de datos
- parameterUpdate(w, b): Actualización de parámetros

### Advices:
- before dataInput(): Validar dimensiones, tipos, valores NaN
- before parameterUpdate(): Validar que parámetros sean finitos

### Reglas de Validación:
- X e y deben tener misma longitud
- No valores NaN o infinitos
- learning_rate > 0
- epochs > 0

## Aspecto 3: HyperparameterAspect
Responsabilidad: Gestionar y adaptar hiperparámetros

### Pointcuts:
- learningRateApplication(learning_rate): Aplicación del learning rate
- gradientCalculation(dw, db): Cálculo de gradientes

### Advices:
- around learningRateApplication(): Learning rate adaptativo basado en magnitud del gradiente
- after gradientCalculation(): Detección de gradientes explosivos/vanishing

### Estrategias:
- Learning rate decay: α = α₀ / (1 + decay_rate * epoch)
- Gradient clipping: dw = max(min(dw, clip_value), -clip_value)

## Aspecto 4: MetricsAspect
Responsabilidad: Calcular y almacenar métricas de performance

### Pointcuts:
- errorCalculation(error): Cálculo del error
- predictionCalculation(y_pred): Cálculo de predicciones

### Advices:
- after errorCalculation(): Calcular MSE, RMSE, MAE
- after predictionCalculation(): Calcular R², accuracy

### Métricas:
- MSE (Mean Squared Error)
- R² (Coeficiente de determinación)
- Tiempo por época
- Velocidad de convergencia

## Aspecto 5: ErrorHandlingAspect
Responsabilidad: Manejo robusto de excepciones

### Pointcuts:
- trainingOperation(): Cualquier operación de entrenamiento
- gradientUpdate(): Operaciones de actualización

### Advices:
- around trainingOperation(): Try-catch alrededor de operaciones críticas
- after throwing Exception(): Log y recuperación de errores

### Estrategias de Recuperación:
- Re-inicialización de parámetros
- Ajuste automático de learning rate
- Fallback a configuración segura

## Aspecto 6: PersistenceAspect
Responsabilidad: Guardar y cargar estados del modelo

### Pointcuts:
- trainingCheckpoint(): Puntos de checkpoint
- modelCompletion(): Finalización del modelo

### Advices:
- after trainingCheckpoint(): Guardar parámetros cada N épocas
- after modelCompletion(): Guardar modelo final

### Datos a Persistir:
- Parámetros w, b
- Historial de métricas
- Hiperparámetros usados
- Metadatos del entrenamiento

## Diagrama de Integración
```text
┌─────────────────────────────────────────────────────────────┐
│                   LinearRegressionCore                       │
├─────────────────────────────────────────────────────────────┤
│  + calculatePredictions()                                   │
│  + calculateError()                                         │
│  + computeGradients()                                       │
│  + updateParameters()                                       │
└─────────────────────────────────────────────────────────────┘
                             │
                             │ Weaving
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                     Aspect Weaver                           │
├─────────────────────────────────────────────────────────────┤
│  │ LoggingAspect        │ ValidationAspect   │             │
│  │ MetricsAspect        │ HyperparameterAspect│            │
│  │ ErrorHandlingAspect  │ PersistenceAspect  │             │
└─────────────────────────────────────────────────────────────┘
``` 

## Flujo de Ejecución con Aspectos

### Inicialización
```text
ValidationAspect → validar datos
LoggingAspect → log inicio
PersistenceAspect → inicializar almacenamiento
```

### Por cada época:
```text
HyperparameterAspect → ajustar learning rate
Core → calcular predicciones
Core → calcular error
MetricsAspect → calcular métricas
Core → calcular gradientes
HyperparameterAspect → aplicar gradient clipping
Core → actualizar parámetros
ValidationAspect → validar parámetros
MetricsAspect → almacenar métricas
LoggingAspect → log progreso
PersistenceAspect → checkpoint (si aplica)
```

### Finalización:
```text
Core → entrenamiento completado
MetricsAspect → calcular métricas finales
PersistenceAspect → guardar modelo
LoggingAspect → log resultados finales
```

## Ventajas del Diseño con Aspectos
- Separación de Concerns: Cada aspecto maneja una preocupación específica
- Modularidad: Los aspectos pueden ser reutilizados en otros algoritmos
- Mantenibilidad: Cambios en logging/validación no afectan el core
- Extensibilidad: Nuevos aspectos pueden añadirse sin modificar el core
- Configurabilidad: Los aspectos pueden activarse/desactivarse según necesidades

## Ejemplo de Configuración
```java
AspectConfiguration config = new AspectConfiguration()
    .enableLogging(level: INFO, frequency: 100)
    .enableValidation(checks: [DATA_INTEGRITY, PARAMETER_SANITY])
    .enableHyperparameterTuning(strategy: ADAPTIVE_LR)
    .enableMetrics(metrics: [MSE, R2, CONVERGENCE_RATE])
    .enablePersistence(checkpoint_interval: 500);
```

Este diseño permite que el algoritmo core de regresión lineal permanezca limpio y enfocado en su lógica principal, mientras que todas las preocupaciones transversales son manejadas de manera modular y reusable a través de aspectos.