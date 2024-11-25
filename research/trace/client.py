"""Connecting to TCP server for receiving messages"""
import sys
import time
import socket

address, port = sys.argv[1].split(":")

port = int(port)
print(f"connecting to {address}:{port}")
client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
while True:
    try:
        client.connect((address, port))
        while True:
            data = client.recv(1)
            if not data:
                print("disconnected!")
                break
            print(data.decode("utf-8"), end="")
            sys.stdout.flush()
    except KeyboardInterrupt:
        print("stopping")
        client.close()
        break;
    except:
        print("cannot connect, retrying...")
        time.sleep(2)
        continue

