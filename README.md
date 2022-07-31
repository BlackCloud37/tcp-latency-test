# TCP latency tester

A simple tool for test latency between TCP client/server.

It sends SYN to server and waits for SYN/ACK, then measure RTTs between SYN and SYN/ACK.

# Usage
> No binary was released yet, you may need to clone this repo and use `cargo build` or `cargo run` to run it,
>     it has been tested on rehl and macos
>     it may requires root permission

`$ ./tcplatency <interface name> <dest ipv4 addr> <dest port> [count]`

For example, test latency from my laptop to bilibili.com(47.103.24.173)'s 443 port:
```
$ ./tcplatency en0 47.103.24.173 443 10
sending 0 SYN packet
send duration 8 us
0 packet receive timeout(10s)
sending 1 SYN packet
send duration 169 us
1 packet rtt: 17353 us
sending 2 SYN packet
send duration 46 us
2 packet rtt: 17549 us
sending 3 SYN packet
send duration 39 us
3 packet rtt: 17527 us
sending 4 SYN packet
send duration 36 us
4 packet rtt: 17393 us
sending 5 SYN packet
send duration 34 us
5 packet rtt: 16951 us
sending 6 SYN packet
send duration 35 us
6 packet rtt: 18041 us
sending 7 SYN packet
send duration 34 us
7 packet rtt: 18084 us
sending 8 SYN packet
send duration 32 us
8 packet rtt: 17144 us
sending 9 SYN packet
send duration 31 us
9 packet rtt: 18465 us
Valid Result Count: 9
RTTs(ns): 17353021,17549396,17527635,17393031,16951531,18041260,18084427,17144146,18465760
AVG RTT(ns): 17612245.222222224
```

In another example, I want to test a lot of ips' latencies, so I wrote a python script to invoke this tester, and collect results as csv:
```python3
import subprocess
import pandas as pd
from tqdm import tqdm

# invoke test command with subprocess and returns avg rtt
def get_rtt(ip, port=443, device='eth0', iter_cnt=5):
    result = subprocess.run(['./tcplatency', device, ip, str(port), str(iter_cnt)], capture_output=True)
    output = result.stdout.decode()
    for line in output.split('\n'):
        if "AVG RTT(ns)" in line:
            return float(line.split(" ")[-1])
    return float("nan")

if __name__ == '__main__':
    iplist = ["47.103.24.173", "47.103.24.173", ]

    results = {ip: [] for ip in iplist}

    # test N=10 times for each ip
    for _ in tqdm(range(10)):
        for ip in iplist:
            results[ip].append(get_rtt(ip))

    df = pd.DataFrame(results)
    df.to_csv('rtts.csv')
```

And the `rtts.csv` will be like:
|Iter  |47.103.24.173|47.103.24.173|47.103.24.173|
|------|------------|------------|-------------|
|0     |74040.4     |794195.2    |846794.8     |
|1     |69776.8     |794098.6    |861836.2     |
|2     |114034.8    |795507.2    |895464.0     |
|3     |80859.0     |788424.0    |840448.8     |
|4     |318998.0    |807142.0    |852057.8     |
|5     |70777.2     |802743.6    |860137.2     |
|6     |87206.4     |791429.0    |857431.6     |
|7     |80243.4     |786114.6    |851218.8     |
|8     |79406.4     |804203.4    |853117.8     |
|9     |79587.8     |799656.6    |846450.6     |
