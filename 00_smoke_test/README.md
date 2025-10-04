# 00 Smoke Test
The requirement is to make a concurrent TCP server that echoes back whatever it is sent.

In my solution I am using io.copy to copy the entire request to the response

The nett.connection struct implements the reader and writer interfaces. 
So reading from the conn object reads from the network and writing to it writes to the network.
