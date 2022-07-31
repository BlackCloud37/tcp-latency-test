# TCP latency tester

A simple tool for test latency between TCP client/server.

It sends SYN to server and wait for SYN/ACK, then measure RTTs between SYN and SYN/ACK.

# Usage

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