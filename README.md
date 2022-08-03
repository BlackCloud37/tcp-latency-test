# TCP Ping

A simple tool for test latency between TCP client/server.

It sends SYN to server and waits for SYN/ACK, then measure RTTs between SYN and SYN/ACK.

# Usage
For mac and linux user, you can download executable binary from release page.

If you cannot run these exes, you can clone this repo and build with `cargo build` (nightly rust environment is required)

```bash
$ ./tcpping --help
tcpping 0.1.0

USAGE:
    tcpping [OPTIONS] --if-name <IF_NAME> --hostname <HOSTNAME>

OPTIONS:
    -c, --count <COUNT>          Number of times to ping [default: 10]
    -h, --hostname <HOSTNAME>    Destination hostname(domain or ipv4 address, ipv6 is not tested)
        --help                   Print help information
    -i, --if-name <IF_NAME>      Local interface
    -p, --port <PORT>            Destination port [default: 80]
    -V, --version                Print version information
```

For example, test latency from my laptop to bilibili.com(47.103.24.173)'s 443 port:
```
$ ./tcpping -i en0 -h 47.103.24.173 -p 443
SYN&ACK(0) from 47.103.24.173 time=26573 us
SYN&ACK(1) from 47.103.24.173 time=18944 us
SYN&ACK(2) from 47.103.24.173 time=15782 us
SYN&ACK(3) from 47.103.24.173 time=28188 us
SYN&ACK(4) from 47.103.24.173 time=17065 us
SYN&ACK(5) from 47.103.24.173 time=15830 us
SYN&ACK(6) from 47.103.24.173 time=27376 us
SYN&ACK(7) from 47.103.24.173 time=25173 us
SYN&ACK(8) from 47.103.24.173 time=79112 us
SYN&ACK(9) from 47.103.24.173 time=55702 us
Valid Result Count: 10
RTTs(us): 26573.354, 18944.521, 15782.49, 28188.198, 17065.896, 15830.354, 27376.864, 25173.698, 79112.698, 55702.5
AVG RTT(us): 30975.0573
```

In another example, I want to test a lot of ips and ports' latencies, so I wrote a python script to invoke this tester, and collect results as csv:
```python3
import subprocess
import pandas as pd
from tqdm import tqdm
import sys
import itertools
# invoke test command with subprocess and returns avg rtt
assert len(sys.argv) >= 2
ofname = sys.argv[1]


def get_rtt(ip, port=443, device='eth0', iter_cnt=1):
    result = subprocess.run(
        ['tcpping', "-i", device, "-h", ip, "-p", str(port), "-c", str(iter_cnt)], capture_output=True)
    output = result.stdout.decode()
    for line in output.split('\n'):
        if "AVG RTT(us)" in line:
            return float(line.split(" ")[-1])
    return float("nan")


def ip_port_to_str(ip_port):
    ip, port = ip_port
    return ip + ":" + str(port)


if __name__ == '__main__':
    with open('iplist.txt', 'r') as f:
        iplist = [l.strip() for l in f.readlines()]
    testcases = list(itertools.product(iplist, (9443, 443)))
    results = {ip_port_to_str(case): [] for case in testcases}

    # test N times for each ip
    for _ in tqdm(range(10)):
        for case in tqdm(testcases, leave=False):
            ip, port = case
            results[ip_port_to_str(case)].append(
                get_rtt(ip, port))

    df = pd.DataFrame(results)
    df.to_csv(ofname)
```

And the `rtts.csv` will looks like:
|  |a.b.c.d:443|a.b.c.d:9443|a.b.c.d:443|
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
