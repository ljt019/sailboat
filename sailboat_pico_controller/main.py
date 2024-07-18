from machine import ADC, Pin, UART
import time

# Initialize ADC on pin GP26 (ADC0)
potentiometer = ADC(Pin(26))

# Initialize UART
uart = UART(0, baudrate=115200)  # UART0, baud rate 115200

while True:
    value = potentiometer.read_u16()
    uart.write(f"{value}\n".encode())  # Encode the string to bytes
    print(f"Value sent: {value}")
    time.sleep(0.1)  # Slightly longer delay to match the receiver's reading speed