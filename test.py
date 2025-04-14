
import socket

# Hex data as a string
hex_data = (
    "000000600001001008ebb5d900096b61666b612d636c6900000001f400000001"
    "0320000000000000000000000002000000000000400080000000000000950200"
    "000000ffffffff0000000000000000ffffffffffffffffff0000100000000001"
    "0100"
)

# Convert hex string to bytes
binary_data = bytes.fromhex(hex_data)

# TCP connection to 127.0.0.1:9092
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(('127.0.0.1', 9092))
    s.sendall(binary_data)
    # Optional: receive response
    response = s.recv(4096)
    print("Received:", response)
