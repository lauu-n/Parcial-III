import numpy as np
import time
import tracemalloc
import matplotlib.pyplot as plt
import psutil
import os

def linear_regression_python(X, y, learning_rate, epochs):
    # Iniciar medición de memoria
    tracemalloc.start()
    start_time = time.time()
    process = psutil.Process(os.getpid())
    
    w, b = 0.0, 0.0
    m = len(X)
    
    # Arrays para guardar métricas durante el entrenamiento
    losses = []
    weights = []
    biases = []
    memory_usage = []
    timestamps = []
    
    for epoch in range(epochs):
        # Predicción y cálculo de error
        y_pred = w * X + b
        error = y_pred - y
        
        # Gradientes
        dw = (2/m) * np.dot(error, X)
        db = (2/m) * np.sum(error)
        
        # Actualización de parámetros
        w -= learning_rate * dw
        b -= learning_rate * db
        
        # Calcular métricas
        mse = np.mean(error ** 2)
        current_memory = tracemalloc.get_traced_memory()[0] / 1024  # KB
        current_time = time.time() - start_time
        
        # Guardar métricas cada 50 epochs
        if epoch % 50 == 0:
            losses.append(mse)
            weights.append(w)
            biases.append(b)
            memory_usage.append(current_memory)
            timestamps.append(current_time)
        
        if (epoch + 1) % 200 == 0:
            print(f"Epoch {epoch+1}, MSE: {mse:.4f}, w: {w:.4f}, b: {b:.4f}")
    
    execution_time = time.time() - start_time
    memory_stats = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    
    return w, b, execution_time, memory_stats, losses, weights, biases, memory_usage, timestamps

def plot_python_metrics(losses, weights, biases, memory_usage, timestamps):
    fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(12, 10))
    
    # Gráfica de pérdida
    epochs_plot = [i * 50 for i in range(len(losses))]
    ax1.plot(epochs_plot, losses, 'b-', linewidth=2)
    ax1.set_title('Evolución del Error (MSE) - Python')
    ax1.set_xlabel('Época')
    ax1.set_ylabel('MSE')
    ax1.grid(True)
    
    # Gráfica de parámetros
    ax2.plot(epochs_plot, weights, 'r-', label='w (pendiente)', linewidth=2)
    ax2.plot(epochs_plot, biases, 'g-', label='b (intercepto)', linewidth=2)
    ax2.set_title('Evolución de Parámetros - Python')
    ax2.set_xlabel('Época')
    ax2.set_ylabel('Valor del Parámetro')
    ax2.legend()
    ax2.grid(True)
    
    # Gráfica de uso de memoria
    ax3.plot(timestamps, memory_usage, 'purple', linewidth=2)
    ax3.set_title('Uso de Memoria - Python')
    ax3.set_xlabel('Tiempo (s)')
    ax3.set_ylabel('Memoria (KB)')
    ax3.grid(True)
    
    # Gráfica de convergencia
    ax4.plot(weights, losses, 'orange', linewidth=2, marker='o', markersize=4)
    ax4.set_title('Convergencia - Python')
    ax4.set_xlabel('Valor de w')
    ax4.set_ylabel('MSE')
    ax4.grid(True)
    
    plt.tight_layout()
    plt.savefig('python_metrics.png', dpi=300, bbox_inches='tight')
    plt.show()

# Ejecutar en Python
print("=== EJECUCIÓN EN PYTHON ===")
X = np.array([1, 2, 3, 4, 5], dtype=float)
y = np.array([2, 4, 6, 8, 10], dtype=float)

w_py, b_py, time_py, memory_py, losses_py, weights_py, biases_py, memory_usage_py, timestamps_py = linear_regression_python(
    X, y, 0.01, 1000
)

print(f"\nResultados Python:")
print(f"w ≈ {w_py:.6f}, b ≈ {b_py:.6f}")
print(f"Tiempo total: {time_py:.6f} segundos")
print(f"Memoria pico: {memory_py[1] / 1024:.2f} KB")

# Generar gráficas para Python
plot_python_metrics(losses_py, weights_py, biases_py, memory_usage_py, timestamps_py)