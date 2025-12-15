import json
import sys
from collections import Counter

class TraceAnalyzer:
    def __init__(self, trace_file):
        self.trace_file = trace_file
        self.events = []

    def load_trace(self):
        """Loads execution trace from a JSON log file (simulated)."""
        print(f"[ANALYSIS] Loading trace: {self.trace_file}")
        try:
            with open(self.trace_file, 'r') as f:
                for line in f:
                    self.events.append(json.loads(line))
        except FileNotFoundError:
            print("[ERROR] Trace file not found.")
            # Generating dummy data for demo purposes if file missing
            self.events = [
                {"pid": 1001, "syscall": "execve", "args": ["./client"], "ret": 0},
                {"pid": 1001, "syscall": "open", "args": ["secret_config.txt"], "ret": 3},
                {"pid": 1001, "syscall": "read", "args": [3], "ret": 1024},
                {"pid": 1001, "syscall": "close", "args": [3], "ret": 0},
                {"pid": 1001, "syscall": "connect", "args": ["192.168.1.5", 80], "ret": -1},
            ]

    def analyze_syscall_frequency(self):
        """Counts syscalls."""
        counts = Counter(e['syscall'] for e in self.events)
        print("\n--- Syscall Frequency ---")
        for syscall, count in counts.most_common():
            print(f"{syscall}: {count}")

    def detect_anomalies(self):
        """Detects suspicious patterns."""
        print("\n--- Anomaly Detection ---")
        for e in self.events:
            if e['syscall'] == "connect" and e['ret'] < 0:
                print(f"[ALERT] Failed network connection attempt by PID {e['pid']}")
            if e['syscall'] == "open" and "secret" in str(e['args']):
                print(f"[WARN] Access to sensitive file detected: {e['args']}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python trace_analyzer.py <trace.json>")
        target = "dummy_trace.json"
    else:
        target = sys.argv[1]

    analyzer = TraceAnalyzer(target)
    analyzer.load_trace()
    analyzer.analyze_syscall_frequency()
    analyzer.detect_anomalies()
