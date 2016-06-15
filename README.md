### NGINX LOGGING

#### One of my first attempts to make Rust work.

This projects aims to read nginx logs and mirror it to a database using async and multi threads processors.
I will use this for cambackup.com to check if a hls stream is active. (I use nginx-rtmp module)

####Usage

build it using `cargo build --release`
- create an environment variable nginx_path like this `export nginx_path='path/to/your/access.log'`
- run the executable like this `./path-to-executable/nglog`


In nginx I have something like this :

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
I have plan to migrate some heavy code that is written in Python on cambackup.com to Rust.

Rust = Faster + less resource usage
