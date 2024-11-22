import socket

HOST = '0.0.0.0'
PORT = 10100
BUFFER_SIZE = 1024

def start_server():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.bind((HOST, PORT))
        server_socket.listen(1)
        print(f"Server listening on {HOST}:{PORT}")

        conn, addr = server_socket.accept()
        with conn:
            conn_alive = True
            print(f"Connected by {addr}")
            while conn_alive:
                try:
                    data = conn.recv(BUFFER_SIZE)
                    if len(data) != 0 and b"fail_me" not in data:
                        print(f"Data: {data}")
                        conn.sendall(b"0 "+data[:len(data):]+b"\r\n")
                        print(f"Received packet: {data}")
                    if b'exit' in data:
                        print("Exit command received")
                        conn_alive = False
                    if b"fail_me" in data:
                        conn.sendall(b"-1234\r\n")
                except KeyboardInterrupt:
                    print("Server shutting down due to keyboard interrupt")
                    break
        print("Server shutting down")
        conn.close()

if __name__ == "__main__":
    start_server()
