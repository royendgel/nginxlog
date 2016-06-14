### NGINX LOGGING

#### One of my first attempts to make Rust work.

This projects aims to read nginx logs and mirror it to a database using async and threads + Multiprocessors.
I will use this for cambackup.com to see if a hls stream is active.

In nginx I have somethin like this :

```
location /hls {
			access_log access.log;
		            # Serve HLS fragments
		            types {
		                application/vnd.apple.mpegurl m3u8;
		            }
		            add_header Cache-Control no-cache;
		        }
```

The log is in standard format.

#####Why Rust language ?
I fell in love with Rust language the same way I felt in love with Python some years ago, and
this is my effort to learn it and do some advanced things like memory management, borrowing.
Rust is a low level programming language that means it is fast (some claims faster then C++).
I think to replace a lot of cambackup.com code with Rust.

Rust = Faster + less resource usage
