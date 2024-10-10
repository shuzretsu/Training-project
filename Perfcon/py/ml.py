import numpy as np
from sklearn.linear_model import LinearRegression
from sklearn.preprocessing import PolynomialFeatures
import matplotlib.pyplot as plt

def analyze_memory(memory_data):
    # Convert to numpy array for processing
    memory_data = np.array(memory_data)

    # Generate X axis (time) values for data
    X = np.arange(len(memory_data)).reshape(-1, 1)
    y = memory_data

    # Create a linear regression model to predict memory usage trend
    linear_model = LinearRegression()
    linear_model.fit(X, y)
    
    # Predict next memory usage trend
    future_time = np.array([[len(memory_data) + i] for i in range(5)])
    future_predictions = linear_model.predict(future_time)

    # Output
    print("Predicted future memory usage:")
    print(future_predictions)

    # Plot the current and predicted data
    plt.scatter(X, y, color='blue', label='Current Data')
    plt.plot(future_time, future_predictions, color='red', label='Predicted')
    plt.legend()
    plt.show()